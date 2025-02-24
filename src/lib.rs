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
static GENERATORS: PgLwLock<FnvIndexMap<String, SharedSnowID, MAX_TABLES>> = PgLwLock::new();

#[pg_guard]
pub extern "C" fn _PG_init() {
    pg_shmem_init!(NODE_ID);
    pg_shmem_init!(GENERATORS);
}

/// Sets the node ID for this PostgreSQL instance
/// This should be unique across your database cluster
#[pg_extern]
fn set_node_id(node: i16) {
    if !(0..=1023).contains(&node) {
        error!("Node ID must be between 0 and 1023");
    }
    NODE_ID.get().store(node, Ordering::Relaxed);
}

/// Gets the currently set node ID
#[pg_extern]
fn get_node_id() -> i16 {
    NODE_ID.get().load(Ordering::Relaxed)
}

/// Generates a new Snowflake ID for the given table
/// Each table gets its own SnowID instance to maintain separate sequences
#[pg_extern]
fn gen_snowid(table_name: &str) -> i64 {
    let mut generators = GENERATORS.exclusive();
    
    // Get or create the generator
    if !generators.contains_key(&table_name.to_string()) {
        let node_id = NODE_ID.get().load(Ordering::Relaxed);
        let snowid = SnowID::new(node_id as u16)
            .unwrap_or_else(|e| error!("Failed to create SnowID generator: {}", e));
        let shared_snowid = SharedSnowID(snowid);
        generators
            .insert(table_name.to_string(), shared_snowid)
            .unwrap();
    }
    
    // Now we can safely get the reference and generate ID while holding the lock
    let generator = &generators[&table_name.to_string()];
    generator.0.generate().try_into().unwrap()
}

/// Extracts the timestamp from a Snowflake ID
#[pg_extern]
fn get_snowid_timestamp(id: i64, table_name: &str) -> i64 {
    let generators = GENERATORS.exclusive();
    let generator = &generators[&table_name.to_string()];
    let id_u64: u64 = id.try_into().unwrap();
    generator.0.extract.timestamp(id_u64).try_into().unwrap()
}

#[pg_extern]
fn snowid_stats() -> String {
    let generators = GENERATORS.share();
    let mut stats = String::from("SnowID Statistics:\n");
    stats.push_str(&format!("Total Generators: {}\n", generators.len()));
    stats.push_str("Generators:\n");

    for (key, _) in generators.iter() {
        stats.push_str(&format!(
            "- Key: '{}', bytes: {:?}\n",
            key,
            key.as_bytes()
        ));
    }

    stats.push_str(&format!(
        "Current Node ID: {}\n\
         Max Tables Supported: {}",
        NODE_ID.get().load(Ordering::Relaxed),
        MAX_TABLES
    ));

    log!("{}", stats);
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
