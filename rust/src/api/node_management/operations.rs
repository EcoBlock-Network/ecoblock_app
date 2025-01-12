use super::storage::NODE_LIST;


/// Add a node to the network.
pub fn add_node(address: String) -> String {
    let mut nodes = NODE_LIST.lock().unwrap();
    if !nodes.contains(&address) {
        nodes.push(address.clone());
        format!("Node {} added successfully.", address)
    } else {
        format!("Node {} already exists.", address)
    }
}

/// Remove a node from the network.
pub fn remove_node(address: String) -> String {
    let mut nodes = NODE_LIST.lock().unwrap();
    if nodes.contains(&address) {
        nodes.retain(|node| node != &address);
        format!("Node {} removed successfully.", address)
    } else {
        format!("Node {} not found in the network.", address)
    }
}

/// Check if a node exists in the network.
pub fn node_exists(address: String) -> bool {
    NODE_LIST.lock().unwrap().contains(&address)
}

/// List all nodes in the network.
pub fn list_nodes() -> Vec<String> {
    NODE_LIST.lock().unwrap().clone()
}