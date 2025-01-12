use std::collections::HashMap;
use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref NODE_PRIORITIES: Mutex<HashMap<String, u8>> = Mutex::new(HashMap::new());
}

/// Set priority for a node.
pub fn set_node_priority(address: String, priority: u8) {
    let mut priorities = NODE_PRIORITIES.lock().unwrap();
    priorities.insert(address, priority);
}

/// Get priority of a node.
pub fn get_node_priority(address: &str) -> Option<u8> {
    let priorities = NODE_PRIORITIES.lock().unwrap();
    priorities.get(address).cloned()
}