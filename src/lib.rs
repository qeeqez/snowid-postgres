use pgrx::prelude::*;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::collections::HashMap;
use snowid::SnowID;

::pgrx::pg_module_magic!();

static NODE_ID: Lazy<Mutex<i32>> = Lazy::new(|| Mutex::new(1));
static GENERATORS: Lazy<Mutex<HashMap<String, SnowID>>> = Lazy::new(|| Mutex::new(HashMap::new()));

/// Sets the node ID for this PostgreSQL instance
/// This should be unique across your database cluster
#[pg_extern]
fn set_node_id(node: i32) {
    if node < 0 || node > 1023 {
        error!("Node ID must be between 0 and 1023");
    }
    let mut node_id = NODE_ID.lock().unwrap();
    *node_id = node;
}

/// Gets the currently set node ID
#[pg_extern]
fn get_node_id() -> i32 {
    *NODE_ID.lock().unwrap()
}

/// Generates a new Snowflake ID for the given table
/// Each table gets its own SnowID instance to maintain separate sequences
#[pg_extern]
fn gen_snowid(table_name: &str) -> i64 {
    let mut generators = GENERATORS.lock().unwrap();
    let node_id = *NODE_ID.lock().unwrap();
    
    let generator = generators.entry(table_name.to_string()).or_insert_with(|| {
        SnowID::new(node_id as u16).unwrap_or_else(|e| {
            error!("Failed to create SnowID generator: {}", e);
        })
    });
    
    generator.generate().try_into().unwrap()
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
    fn test_gen_snowid() {
        // Test generating IDs for different tables
        crate::set_node_id(1);
        let id1 = crate::gen_snowid("table1");
        let id2 = crate::gen_snowid("table1");
        let id3 = crate::gen_snowid("table2");
        
        // IDs should be different
        assert_ne!(id1, id2);
        assert_ne!(id1, id3);
        assert_ne!(id2, id3);
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