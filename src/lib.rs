use heapless::FnvIndexMap;
use pgrx::atomics::*;
use pgrx::lwlock::PgLwLock;
use pgrx::pg_shmem_init;
use pgrx::prelude::*;
use pgrx::shmem::PGRXSharedMemory;
use pgrx::shmem::*;
use snowid::SnowID;
use std::sync::atomic::{AtomicI16, Ordering}; // Import PgSharedMemory directly

::pgrx::pg_module_magic!();

const MAX_TABLES: usize = 1024;

#[derive(Debug, Clone, Copy)]
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
    if node < 0 || node > 1023 {
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
fn gen_snowid(table_name: String) -> i64 {
    let mut generators = GENERATORS.exclusive();

    let mut generator: SharedSnowID;
    if !generators.contains_key(&table_name) {
        let node_id = NODE_ID.get().load(Ordering::Relaxed);
        let snowid: SnowID = SnowID::new(node_id as u16)
            .unwrap_or_else(|e| error!("Failed to create SnowID generator: {}", e));
        let shared_snowid = SharedSnowID(snowid);
        generator = generators
            .insert(table_name, shared_snowid)
            .unwrap()
            .unwrap()
    } else {
        generator = generators[&table_name]
    }

    generator.0.generate().try_into().unwrap()
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgrx::prelude::*;

    #[pg_test]
    fn test_node_id() {
        // Test setting and getting node ID
        crate::set_node_id(5);
        assert_eq!(5, crate::get_node_id());
    }

    #[pg_test]
    fn test_gen_snowid_not_equal() {
        // Test generating IDs for different tables
        crate::set_node_id(1);
        let id1 = crate::gen_snowid("table1");
        let id2 = crate::gen_snowid("table1");
        let id3 = crate::gen_snowid("table1");

        // IDs should be different
        assert_ne!(id1, id2);
        assert_ne!(id1, id3);
        assert_ne!(id2, id3);
    }

    #[pg_test]
    fn test_gen_snowid_equal() {
        // Test generating IDs for different tables
        crate::set_node_id(1);
        let id1 = crate::gen_snowid("table1");
        let id2 = crate::gen_snowid("table2");
        let id3 = crate::gen_snowid("table3");

        // IDs should be different
        assert_eq!(id1, id2);
        assert_eq!(id1, id3);
        assert_eq!(id2, id3);
    }

    #[pg_test]
    fn test_get_snowid_timestamp() {
        // Test generating IDs for different tables
        crate::set_node_id(1);
        let timestamp = crate::get_snowid_timestamp(151900616753418240, "table1");

        // IDs should be different
        assert_eq!(timestamp, 36215929211);
    }
}

/// This module is required by `cargo pgrx test` invocations.
/// It must be visible at the root of your extension crate.
#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    pub fn postgresql_conf_options() -> Vec<&'static str> {
        vec![]
    }
}