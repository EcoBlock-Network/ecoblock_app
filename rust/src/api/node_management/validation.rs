use std::net::TcpStream;

/// Validate if the given address is in a valid format.
pub fn validate_address(address: &str) -> bool {
    address.parse::<std::net::SocketAddr>().is_ok()
}

/// Check if a node is reachable by attempting a TCP connection.
pub fn ping_node(address: String) -> bool {
    TcpStream::connect(address).is_ok()
}