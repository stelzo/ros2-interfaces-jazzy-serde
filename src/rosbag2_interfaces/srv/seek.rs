use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SeekRequest {
    pub time: crate::builtin_interfaces::msg::Time,
}

impl Default for SeekRequest {
    fn default() -> Self {
        SeekRequest {
            time: crate::builtin_interfaces::msg::Time::default(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for SeekRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SeekResponse {
    pub success: bool,
}

impl Default for SeekResponse {
    fn default() -> Self {
        SeekResponse {
            success: false,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for SeekResponse {}


pub struct Seek;
