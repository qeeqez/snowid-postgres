#![allow(unexpected_cfgs)]

use heapless::index_map::FnvIndexMap;
use pgrx::atomics::PgAtomic;
use pgrx::lwlock::PgLwLock;
use pgrx::pg_shmem_init;
use pgrx::prelude::{error, pg_extern, pg_guard, pg_module_magic, pg_sys};
use pgrx::shmem::AssertPGRXSharedMemory;
use pgrx::shmem::PGRXSharedMemory;
use snowid::{SnowID, base62};
use std::fmt::Write as _;
use std::sync::atomic::{AtomicI16, Ordering};

pg_module_magic!();

const MAX_TABLES: usize = 1024;
const MAX_BATCH_SIZE: i32 = 100_000;

#[derive(Debug)]
struct SharedSnowID(SnowID);

unsafe impl PGRXSharedMemory for SharedSnowID {}

impl Default for SharedSnowID {
    fn default() -> Self {
        Self(SnowID::new(1).unwrap())
    }
}

// SAFETY: C-string literals are null-terminated and valid for static initialization
static NODE_ID: PgAtomic<AtomicI16> = unsafe { PgAtomic::new(c"NODE_ID") };
static GENERATORS: PgLwLock<AssertPGRXSharedMemory<FnvIndexMap<i32, SharedSnowID, MAX_TABLES>>> = unsafe { PgLwLock::new(c"GENERATORS") };

#[pg_guard]
pub extern "C-unwind" fn _PG_init() {
    pg_shmem_init!(NODE_ID);
    // heapless containers require explicit initialization via AssertPGRXSharedMemory wrapper
    pg_shmem_init!(GENERATORS = unsafe { AssertPGRXSharedMemory::new(FnvIndexMap::default()) });
}

/// Sets node ID (0-1023) for this `PostgreSQL` instance
///
/// @param node - Node ID between 0 and 1023
/// @example SELECT `snowid_set_node`(5);
#[pg_extern]
fn snowid_set_node(node: i16) {
    if !(0..=1023).contains(&node) {
        error!("Node ID must be between 0 and 1023");
    }
    NODE_ID.get().store(node, Ordering::Relaxed);
}

/// Gets current node ID
///
/// @returns Node ID (0-1023)
/// @example SELECT `snowid_get_node`();
#[pg_extern]
fn snowid_get_node() -> i16 {
    NODE_ID.get().load(Ordering::Relaxed)
}

/// Generates unique `SnowID` for given table
///
/// Uses SnowID's default logical timestamp generation. When the current
/// millisecond's sequence range is exhausted, generation advances the timestamp
/// component instead of waiting for the next wall-clock millisecond.
///
/// @param `table_id` - Unique positive integer ID for the table
/// @returns 64-bit unique time-sorted identifier
/// @example CREATE TABLE users (id bigint PRIMARY KEY DEFAULT `snowid_generate`(1));
#[pg_extern]
fn snowid_generate(table_id: pg_sys::Oid) -> i64 {
    let table_id = table_oid_to_id(table_id);
    with_table_generator(table_id, |sid| sid.generate().try_into().unwrap())
}

/// Tries to generate a unique `SnowID` for given table without logical timestamp advancement
///
/// Returns `NULL` instead of advancing the logical timestamp when the current
/// wall-clock millisecond's sequence range is exhausted or the generator's
/// logical timestamp is already ahead of wall-clock time.
///
/// @param `table_id` - Unique positive integer ID for the table
/// @returns 64-bit unique time-sorted identifier, or NULL when unavailable immediately
/// @example SELECT `snowid_try_generate`(1);
#[pg_extern]
fn snowid_try_generate(table_id: pg_sys::Oid) -> Option<i64> {
    let table_id = table_oid_to_id(table_id);
    with_table_generator(table_id, |sid| sid.try_generate().ok().map(|id| id.try_into().unwrap()))
}

/// Generates a batch of unique `SnowID`s for given table
///
/// Uses SnowID's logical batch reservation. It always returns `count` IDs and
/// may advance timestamp components instead of waiting under sustained load.
///
/// @param `table_id` - Unique positive integer ID for the table
/// @param count - Number of IDs to generate
/// @returns Array of 64-bit unique time-sorted identifiers
/// @example SELECT unnest(`snowid_generate_batch`(1, 1000));
#[pg_extern]
fn snowid_generate_batch(table_id: pg_sys::Oid, count: i32) -> Vec<i64> {
    let table_id = table_oid_to_id(table_id);
    let count = validate_batch_count(count);
    let mut ids = vec![0_u64; count];
    with_table_generator(table_id, |sid| {
        sid.generate_batch(&mut ids);
    });
    ids.into_iter().map(|id| id.try_into().unwrap()).collect()
}

/// Tries to generate a batch of unique `SnowID`s without logical timestamp advancement
///
/// Returns as many IDs as can be reserved immediately from the wall-clock
/// millisecond. The returned array can contain fewer than `count` IDs.
///
/// @param `table_id` - Unique positive integer ID for the table
/// @param count - Maximum number of IDs to generate
/// @returns Array of immediately available 64-bit unique time-sorted identifiers
/// @example SELECT unnest(`snowid_try_generate_batch`(1, 1000));
#[pg_extern]
fn snowid_try_generate_batch(table_id: pg_sys::Oid, count: i32) -> Vec<i64> {
    let table_id = table_oid_to_id(table_id);
    let count = validate_batch_count(count);
    let mut ids = vec![0_u64; count];
    let written = with_table_generator(table_id, |sid| sid.try_generate_batch(&mut ids));
    ids.truncate(written);
    ids.into_iter().map(|id| id.try_into().unwrap()).collect()
}

/// Generates unique base62-encoded `SnowID` for given table
///
/// Uses the same logical timestamp generation behavior as `snowid_generate`.
///
/// @param `table_id` - Unique positive integer ID for the table
/// @returns base62-encoded unique time-sorted identifier (VARCHAR(11))
/// @example CREATE TABLE users (id VARCHAR(11) PRIMARY KEY DEFAULT `snowid_generate_base62`(1));
#[pg_extern]
fn snowid_generate_base62(table_id: pg_sys::Oid) -> String {
    let table_id = table_oid_to_id(table_id);
    with_table_generator(table_id, SnowID::generate_base62)
}

/// Tries to generate a base62-encoded `SnowID` without logical timestamp advancement
///
/// Returns `NULL` instead of advancing the logical timestamp when an ID cannot
/// be generated immediately from wall-clock time.
///
/// @param `table_id` - Unique positive integer ID for the table
/// @returns base62-encoded unique time-sorted identifier, or NULL when unavailable immediately
/// @example SELECT `snowid_try_generate_base62`(1);
#[pg_extern]
fn snowid_try_generate_base62(table_id: pg_sys::Oid) -> Option<String> {
    let table_id = table_oid_to_id(table_id);
    with_table_generator(table_id, |sid| sid.try_generate().ok().map(base62::encode))
}

fn table_oid_to_id(table_id: pg_sys::Oid) -> i32 {
    let table_id = table_id.to_u32().try_into().unwrap();
    if table_id <= 0 {
        error!("Table ID must be a positive number");
    }
    table_id
}

fn validate_batch_count(count: i32) -> usize {
    if count < 0 {
        error!("Batch count must be non-negative");
    }
    if count > MAX_BATCH_SIZE {
        error!("Batch count must not exceed {}", MAX_BATCH_SIZE);
    }
    usize::try_from(count).unwrap()
}

/// Helper function to create a generator for a table
fn create_generator_for_table(generators: &mut FnvIndexMap<i32, SharedSnowID, MAX_TABLES>, table_id: i32) {
    let node_id = NODE_ID.get().load(Ordering::Relaxed);
    let Ok(snowid) = SnowID::new(u16::try_from(node_id).unwrap()) else {
        error!("Failed to create SnowID generator for node {}", node_id);
    };
    let shared_snowid = SharedSnowID(snowid);
    if generators.insert(table_id, shared_snowid).is_err() {
        error!("Failed to insert generator for table ID {}, map is full", table_id);
    }
}

/// Runs the provided function with a generator for the given table id.
/// Creates the generator if it doesn't exist using a double-checked locking pattern.
fn with_table_generator<R>(table_id: i32, mut f: impl FnMut(&SnowID) -> R) -> R {
    // Fast path under shared lock
    let generators_shared = GENERATORS.share();
    if let Some(generator) = generators_shared.get(&table_id) {
        return f(&generator.0);
    }
    drop(generators_shared);

    // Slow path: create under exclusive lock if still absent
    let mut generators = GENERATORS.exclusive();
    if !generators.contains_key(&table_id) {
        create_generator_for_table(&mut generators, table_id);
    }
    f(&generators[&table_id].0)
}

/// Gets timestamp from `SnowID`
///
/// @param id - Snowflake ID
/// @returns Unix timestamp in milliseconds
/// @example SELECT `snowid_get_timestamp`(151819733950271234);
#[pg_extern]
fn snowid_get_timestamp(id: i64) -> i64 {
    if id < 0 {
        error!("ID must be non-negative");
    }
    let id_u64: u64 = u64::try_from(id).unwrap();

    with_any_generator(|sid| sid.extract.timestamp(id_u64).try_into().unwrap())
}

/// Gets timestamp from base62-encoded `SnowID`
///
/// @param `encoded_id` - Base62-encoded `SnowID`
/// @returns Unix timestamp in milliseconds
/// @example SELECT `snowid_get_timestamp_base62`('2qPfVQh7Jw9');
#[pg_extern]
fn snowid_get_timestamp_base62(encoded_id: &str) -> i64 {
    with_any_generator(|sid| match sid.decode_base62(encoded_id) {
        Ok(id) => sid.extract.timestamp(id).try_into().unwrap(),
        Err(e) => error!("Failed to decode base62 ID: {}", e),
    })
}

/// Ensures there is at least one generator and runs the provided function with it.
fn with_any_generator<R>(f: impl Fn(&SnowID) -> R) -> R {
    // Fast path under shared lock
    let generators_shared = GENERATORS.share();
    if let Some((_, generator)) = generators_shared.iter().next() {
        return f(&generator.0);
    }
    drop(generators_shared);

    // Slow path: create a default generator under exclusive lock if needed
    let mut generators = GENERATORS.exclusive();
    if generators.is_empty() {
        let node_id = NODE_ID.get().load(Ordering::Relaxed);
        let Ok(snowid) = SnowID::new(u16::try_from(node_id).unwrap()) else {
            error!("Failed to create default generator for node {}", node_id);
        };
        let shared_snowid = SharedSnowID(snowid);
        if generators.insert(0, shared_snowid).is_err() {
            error!("Failed to insert default generator");
        }
    }

    let (_, generator) = generators.iter().next().unwrap();
    f(&generator.0)
}

/// Shows `SnowID` statistics (generators, `table_id`s, node ID)
///
/// @returns Statistics about `SnowID` usage
/// @example SELECT `snowid_stats`();
#[pg_extern]
fn snowid_stats() -> String {
    let generators = GENERATORS.share();
    let mut stats = String::from("SnowID Statistics:\n");
    let _ = writeln!(stats, "Total Generators: {}", generators.len());
    let _ = writeln!(stats, "Generators:");

    for (table_id, _) in generators.iter() {
        let _ = writeln!(stats, "- Table ID: {table_id}");
    }

    let _ = write!(
        stats,
        "Current Node ID: {}\n\
         Max Tables Supported: {}",
        snowid_get_node(),
        MAX_TABLES
    );
    stats
}

/// This module is required by `cargo pgrx test` invocations.
/// It must be visible at the root of your extension crate.
#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    #[must_use]
    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}
