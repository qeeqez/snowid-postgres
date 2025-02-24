#![allow(unexpected_cfgs)]

use heapless::FnvIndexMap;
use pgrx::atomics::*;
use pgrx::lwlock::PgLwLock;
use pgrx::pg_shmem_init;
use pgrx::prelude::*;
use pgrx::shmem::PGRXSharedMemory;
use pgrx::shmem::*;
use snowid::SnowID;
use std::sync::atomic::{AtomicI16, Ordering}; // Import PgSharedMemory directly

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

static NODE_ID: PgAtomic<AtomicI16> = PgAtomic::new();
static GENERATORS: PgLwLock<FnvIndexMap<i32, SharedSnowID, MAX_TABLES>> = PgLwLock::new();

#[pg_guard]
pub extern "C" fn _PG_init() {
    pg_shmem_init!(NODE_ID);
    pg_shmem_init!(GENERATORS);
}

/// Sets the node ID for this PostgreSQL instance
/// This should be unique across your database cluster
#[pg_extern]
fn snowid_set_node(node: i16) {
    if !(0..=1023).contains(&node) {
        error!("Node ID must be between 0 and 1023");
    }
    NODE_ID.get().store(node, Ordering::Relaxed);
}

/// Gets the currently set node ID
#[pg_extern]
fn snowid_get_node() -> i16 {
    NODE_ID.get().load(Ordering::Relaxed)
}

/// Generates a new Snowflake ID for the given table
/// Each table gets its own SnowID instance to maintain separate sequences
/// table_id should be a unique number for each table, preferably starting from 1
#[pg_extern]
fn snowid_generate(table_id: i32) -> i64 {
    if table_id <= 0 {
        error!("Table ID must be a positive number");
    }
    
    let mut generators = GENERATORS.exclusive();
    
    // Get or create the generator
    if !generators.contains_key(&table_id) {
        let node_id = NODE_ID.get().load(Ordering::Relaxed);
        let snowid = SnowID::new(node_id as u16)
            .unwrap_or_else(|e| error!("Failed to create SnowID generator: {}", e));
        let shared_snowid = SharedSnowID(snowid);
        generators
            .insert(table_id, shared_snowid)
            .unwrap_or_else(|_| error!("Failed to insert generator for table ID {}", table_id));
    }
    
    // Now we can safely get the reference and generate ID while holding the lock
    let generator = &generators[&table_id];
    generator.0.generate().try_into().unwrap()
}

/// Extracts the timestamp from a Snowflake ID
#[pg_extern]
fn snowid_get_timestamp(id: i64, table_id: i32) -> i64 {
    if table_id <= 0 {
        error!("Table ID must be a positive number");
    }
    
    let generators = GENERATORS.exclusive();
    let generator = generators
        .get(&table_id)
        .unwrap_or_else(|| error!("No generator found for table ID {}", table_id));
    let id_u64: u64 = id.try_into().unwrap();
    generator.0.extract.timestamp(id_u64).try_into().unwrap()
}

#[pg_extern]
fn snowid_stats() -> String {
    let generators = GENERATORS.share();
    let mut stats = String::from("SnowID Statistics:\n");
    stats.push_str(&format!("Total Generators: {}\n", generators.len()));
    stats.push_str("Generators:\n");

    for (table_id, _) in generators.iter() {
        stats.push_str(&format!(
            "- Table ID: {}\n",
            table_id
        ));
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
