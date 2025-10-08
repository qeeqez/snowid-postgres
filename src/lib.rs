#![allow(unexpected_cfgs)]

use heapless::index_map::FnvIndexMap;
use pgrx::atomics::PgAtomic;
use pgrx::lwlock::PgLwLock;
use pgrx::pg_shmem_init;
use pgrx::prelude::*;
use pgrx::shmem::PGRXSharedMemory;
use pgrx::shmem::AssertPGRXSharedMemory;
use snowid::SnowID;
use std::sync::atomic::{AtomicI16, Ordering};

pg_module_magic!();

const MAX_TABLES: usize = 1024;

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
static GENERATORS: PgLwLock<AssertPGRXSharedMemory<FnvIndexMap<i32, SharedSnowID, MAX_TABLES>>> =
    unsafe { PgLwLock::new(c"GENERATORS") };

#[pg_guard]
pub extern "C-unwind" fn _PG_init() {
    pg_shmem_init!(NODE_ID);
    // heapless containers require explicit initialization via AssertPGRXSharedMemory wrapper
    pg_shmem_init!(GENERATORS = unsafe { AssertPGRXSharedMemory::new(Default::default()) });
}

/// Sets node ID (0-1023) for this PostgreSQL instance
///
/// @param node - Node ID between 0 and 1023
/// @example SELECT snowid_set_node(5);
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
/// @example SELECT snowid_get_node();
#[pg_extern]
fn snowid_get_node() -> i16 {
    NODE_ID.get().load(Ordering::Relaxed)
}

#[pg_extern]
fn snowid_generate(table_id: pg_sys::Oid) -> i64 {
    snowid_generate_int(table_id.to_u32() as i32)
}

/// Generates unique Snowflake ID for given table
///
/// @param table_id - Unique positive integer ID for the table
/// @returns 64-bit unique time-sorted identifier
/// @example CREATE TABLE users (id bigint PRIMARY KEY DEFAULT snowid_generate(1));
#[pg_extern]
fn snowid_generate_int(table_id: i32) -> i64 {
    if table_id <= 0 {
        error!("Table ID must be a positive number");
    }

    with_table_generator(table_id, |sid| sid.generate().try_into().unwrap())
}

#[pg_extern]
fn snowid_generate_base62(table_id: pg_sys::Oid) -> String {
    snowid_generate_base62_int(table_id.to_u32() as i32)
}

/// Generates unique base62-encoded Snowflake ID for given table
///
/// @param table_id - Unique positive integer ID for the table
/// @returns base62-encoded unique time-sorted identifier (VARCHAR(11))
/// @example CREATE TABLE users (id VARCHAR(11) PRIMARY KEY DEFAULT snowid_generate_base62(1));
#[pg_extern]
fn snowid_generate_base62_int(table_id: i32) -> String {
    if table_id <= 0 {
        error!("Table ID must be a positive number");
    }

    with_table_generator(table_id, |sid| sid.generate_base62())
}

/// Helper function to create a generator for a table
fn create_generator_for_table(
    generators: &mut FnvIndexMap<i32, SharedSnowID, MAX_TABLES>,
    table_id: i32,
) {
    let node_id = NODE_ID.get().load(Ordering::Relaxed);
    let Ok(snowid) = SnowID::new(node_id as u16) else {
        error!("Failed to create SnowID generator for node {}", node_id);
    };
    let shared_snowid = SharedSnowID(snowid);
    if generators.insert(table_id, shared_snowid).is_err() {
        error!(
            "Failed to insert generator for table ID {}, map is full",
            table_id
        );
    }
}

/// Runs the provided function with a generator for the given table id.
/// Creates the generator if it doesn't exist using a double-checked locking pattern.
fn with_table_generator<R>(table_id: i32, f: impl Fn(&SnowID) -> R) -> R {
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

/// Gets timestamp from Snowflake ID
///
/// @param id - Snowflake ID
/// @returns Unix timestamp in milliseconds
/// @example SELECT snowid_get_timestamp(151819733950271234);
#[pg_extern]
fn snowid_get_timestamp(id: i64) -> i64 {
    if id < 0 {
        error!("ID must be non-negative");
    }
    let id_u64: u64 = id as u64;

    with_any_generator(|sid| sid.extract.timestamp(id_u64).try_into().unwrap())
}

/// Gets timestamp from base62-encoded Snowflake ID
///
/// @param encoded_id - Base62-encoded Snowflake ID
/// @returns Unix timestamp in milliseconds
/// @example SELECT snowid_get_timestamp_base62('2qPfVQh7Jw9');
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
        let Ok(snowid) = SnowID::new(node_id as u16) else {
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

/// Shows SnowID statistics (generators, table IDs, node ID)
///
/// @returns Statistics about SnowID usage
/// @example SELECT snowid_stats();
#[pg_extern]
fn snowid_stats() -> String {
    let generators = GENERATORS.share();
    let mut stats = String::from("SnowID Statistics:\n");
    stats.push_str(&format!("Total Generators: {}\n", generators.len()));
    stats.push_str("Generators:\n");

    for (table_id, _) in generators.iter() {
        stats.push_str(&format!("- Table ID: {}\n", table_id));
    }

    stats.push_str(&format!(
        "Current Node ID: {}\n\
         Max Tables Supported: {}",
        snowid_get_node(),
        MAX_TABLES
    ));
    stats
}

/// This module is required by `cargo pgrx test` invocations.
/// It must be visible at the root of your extension crate.
#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}
