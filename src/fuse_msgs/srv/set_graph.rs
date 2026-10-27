use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetGraphRequest {
    pub graph: crate::fuse_msgs::msg::SerializedGraph,
}

impl Default for SetGraphRequest {
    fn default() -> Self {
        SetGraphRequest {
            graph: crate::fuse_msgs::msg::SerializedGraph::default(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for SetGraphRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetGraphResponse {
    pub success: bool,
    pub message: ::std::string::String,
}

impl Default for SetGraphResponse {
    fn default() -> Self {
        SetGraphResponse {
            success: false,
            message: ::std::string::String::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for SetGraphResponse {}


pub struct SetGraph;
