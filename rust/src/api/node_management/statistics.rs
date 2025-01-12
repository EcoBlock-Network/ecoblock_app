use super::storage::NODE_LIST;


/// Get statistics about the nodes in the network.
pub fn get_node_statistics() -> (usize, Vec<String>) {
    let nodes = NODE_LIST.lock().unwrap();
    let count = nodes.len();
    let mut sorted_nodes = nodes.clone();
    sorted_nodes.sort();
    (count, sorted_nodes)
}

/// Search for nodes matching a given prefix.
pub fn search_nodes(prefix: &str) -> Vec<String> {
    NODE_LIST.lock().unwrap()
        .iter()
        .filter(|node| node.starts_with(prefix))
        .cloned()
        .collect()
}