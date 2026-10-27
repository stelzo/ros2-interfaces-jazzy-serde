use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenSnapshotStreamRequest {
    pub topic_name: ::std::string::String,
    pub parameters: crate::py_trees_ros_interfaces::msg::SnapshotStreamParameters,
}

impl Default for OpenSnapshotStreamRequest {
    fn default() -> Self {
        OpenSnapshotStreamRequest {
            topic_name: ::std::string::String::new(),
            parameters: crate::py_trees_ros_interfaces::msg::SnapshotStreamParameters::default(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for OpenSnapshotStreamRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenSnapshotStreamResponse {
    pub topic_name: ::std::string::String,
}

impl Default for OpenSnapshotStreamResponse {
    fn default() -> Self {
        OpenSnapshotStreamResponse {
            topic_name: ::std::string::String::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for OpenSnapshotStreamResponse {}


pub struct OpenSnapshotStream;
