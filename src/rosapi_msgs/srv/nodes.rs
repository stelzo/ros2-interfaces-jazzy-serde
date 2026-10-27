use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NodesRequest {

}

impl Default for NodesRequest {
    fn default() -> Self {
        NodesRequest {

        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for NodesRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NodesResponse {
    pub nodes: Vec<::std::string::String>,
}

impl Default for NodesResponse {
    fn default() -> Self {
        NodesResponse {
            nodes: Vec::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for NodesResponse {}


pub struct Nodes;
