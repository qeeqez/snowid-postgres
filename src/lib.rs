#![allow(unexpected_cfgs)]

use heapless::FnvIndexMap;
use pgrx::atomics::PgAtomic;
use pgrx::lwlock::PgLwLock;
use pgrx::pg_shmem_init;
use pgrx::prelude::*;
use pgrx::shmem::PGRXSharedMemory;
use pgrx::shmem::*;
use snowid::SnowID;
use std::ffi::CStr;
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

static NODE_ID: PgAtomic<AtomicI16> =
    PgAtomic::new(unsafe { CStr::from_bytes_with_nul_unchecked(b"NODE_ID\0") });
static GENERATORS: PgLwLock<FnvIndexMap<i32, SharedSnowID, MAX_TABLES>> =
    PgLwLock::new(unsafe { CStr::from_bytes_with_nul_unchecked(b"GENERATORS\0") });

#[pg_guard]
pub extern "C-unwind" fn _PG_init() {
    pg_shmem_init!(NODE_ID);
    pg_shmem_init!(GENERATORS);
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

    // First check with shared lock if the generator exists
    let generators_shared = GENERATORS.share();
    if generators_shared.contains_key(&table_id) {
        // Fast path: generator exists, generate ID while holding shared lock
        return generators_shared[&table_id]
            .0
            .generate()
            .try_into()
            .unwrap();
    }

    // Drop shared lock before acquiring exclusive lock to prevent deadlocks
    drop(generators_shared);

    // Slow path: need to create generator, take exclusive lock
    let mut generators = GENERATORS.exclusive();

    // Double-check after acquiring exclusive lock (another session might have created it)
    if !generators.contains_key(&table_id) {
        create_generator_for_table(&mut generators, table_id);
    }

    // Now we can safely get the reference and generate ID while holding the lock
    generators[&table_id].0.generate().try_into().unwrap()
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

    // First check with shared lock if the generator exists
    let generators_shared = GENERATORS.share();
    if generators_shared.contains_key(&table_id) {
        // Fast path: generator exists, generate ID while holding shared lock
        return generators_shared[&table_id].0.generate_base62();
    }

    // Drop shared lock before acquiring exclusive lock to prevent deadlocks
    drop(generators_shared);

    // Slow path: need to create generator, take exclusive lock
    let mut generators = GENERATORS.exclusive();

    // Double-check after acquiring exclusive lock (another session might have created it)
    if !generators.contains_key(&table_id) {
        create_generator_for_table(&mut generators, table_id);
    }

    // Now we can safely get the reference and generate ID while holding the lock
    generators[&table_id].0.generate_base62()
}

/// Helper function to create a generator for a table
fn create_generator_for_table(
    generators: &mut FnvIndexMap<i32, SharedSnowID, MAX_TABLES>,
    table_id: i32,
) {
    let node_id = NODE_ID.get().load(Ordering::Relaxed);
    let snowid = match SnowID::new(node_id as u16) {
        Ok(id) => id,
        Err(e) => error!("Failed to create SnowID generator: {}", e),
    };
    let shared_snowid = SharedSnowID(snowid);
    if let Err(_) = generators.insert(table_id, shared_snowid) {
        error!(
            "Failed to insert generator for table ID {}, map is full",
            table_id
        );
    }
}

/// Gets timestamp from Snowflake ID
///
/// @param id - Snowflake ID
/// @returns Unix timestamp in milliseconds
/// @example SELECT snowid_get_timestamp(151819733950271234);
#[pg_extern]
fn snowid_get_timestamp(id: i64) -> i64 {
    let id_u64: u64 = id.try_into().unwrap();

    let generators_shared = GENERATORS.share();
    // Use any existing generator, doesn't matter which one
    if !generators_shared.is_empty() {
        let (_, generator) = generators_shared.iter().next().unwrap();
        return generator.0.extract.timestamp(id_u64).try_into().unwrap();
    }

    // If no generators exist, create a default one
    let mut generators = GENERATORS.exclusive();
    if generators.is_empty() {
        let node_id = NODE_ID.get().load(Ordering::Relaxed);
        match SnowID::new(node_id as u16) {
            Ok(snowid) => {
                let shared_snowid = SharedSnowID(snowid);
                if let Err(_) = generators.insert(0, shared_snowid) {
                    error!("Failed to insert default generator");
                }
            }
            Err(e) => error!("Failed to create default generator: {}", e),
        }
    }

    let (_, generator) = generators.iter().next().unwrap();
    generator.0.extract.timestamp(id_u64).try_into().unwrap()
}

/// Gets timestamp from base62-encoded Snowflake ID
///
/// @param encoded_id - Base62-encoded Snowflake ID
/// @returns Unix timestamp in milliseconds
/// @example SELECT snowid_get_timestamp_base62('2qPfVQh7Jw9');
#[pg_extern]
fn snowid_get_timestamp_base62(encoded_id: &str) -> i64 {
    let generators_shared = GENERATORS.share();

    // Use any existing generator, doesn't matter which one
    if !generators_shared.is_empty() {
        let (_, generator) = generators_shared.iter().next().unwrap();
        match generator.0.decode_base62(encoded_id) {
            Ok(id) => return generator.0.extract.timestamp(id).try_into().unwrap(),
            Err(e) => error!("Failed to decode base62 ID: {}", e),
        }
    }

    // If no generators exist, create a default one
    let mut generators = GENERATORS.exclusive();
    if generators.is_empty() {
        let node_id = NODE_ID.get().load(Ordering::Relaxed);
        match SnowID::new(node_id as u16) {
            Ok(snowid) => {
                let shared_snowid = SharedSnowID(snowid);
                if let Err(_) = generators.insert(0, shared_snowid) {
                    error!("Failed to insert default generator");
                }
            }
            Err(e) => error!("Failed to create default generator: {}", e),
        }
    }

    let (_, generator) = generators.iter().next().unwrap();
    match generator.0.decode_base62(encoded_id) {
        Ok(id) => generator.0.extract.timestamp(id).try_into().unwrap(),
        Err(e) => error!("Failed to decode base62 ID: {}", e),
    }
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
