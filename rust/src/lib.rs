pub mod api;
mod frb_generated;


#[flutter_rust_bridge::frb]
pub fn check_connectivity(address: String) -> Result<bool, String> {
    crate::api::node_management::operations::check_node_connectivity(&address)
}