#![allow(unexpected_cfgs)]

use heapless::FnvIndexMap;
use pgrx::atomics::*;
use pgrx::lwlock::PgLwLock;
use pgrx::pg_shmem_init;
use pgrx::prelude::*;
use pgrx::shmem::PGRXSharedMemory;
use pgrx::shmem::*;
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

static NODE_ID: PgAtomic<AtomicI16> = PgAtomic::new();
static GENERATORS: PgLwLock<FnvIndexMap<i32, SharedSnowID, MAX_TABLES>> = PgLwLock::new();

#[pg_guard]
pub extern "C" fn _PG_init() {
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

/// Generates unique Snowflake ID for given table
/// 
/// @param table_id - Unique positive integer ID for the table
/// @returns 64-bit unique time-sorted identifier
/// @example CREATE TABLE users (id bigint PRIMARY KEY DEFAULT snowid_generate(1));
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

/// Gets timestamp from Snowflake ID
/// 
/// @param id - Snowflake ID
/// @returns Unix timestamp in milliseconds
/// @example SELECT snowid_get_timestamp(151819733950271234);
#[pg_extern]
fn snowid_get_timestamp(id: i64) -> i64 {
    let mut generators = GENERATORS.exclusive();
    
    // Use default generator (table_id = 0) for timestamp extraction
    if !generators.contains_key(&0) {
        let node_id = NODE_ID.get().load(Ordering::Relaxed);
        let snowid = SnowID::new(node_id as u16)
            .unwrap_or_else(|e| error!("Failed to create default generator: {}", e));
        let shared_snowid = SharedSnowID(snowid);
        generators
            .insert(0, shared_snowid)
            .unwrap_or_else(|_| error!("Failed to insert default generator"));
    }
    
    let generator = &generators[&0];
    let id_u64: u64 = id.try_into().unwrap();
    generator.0.extract.timestamp(id_u64).try_into().unwrap()
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
