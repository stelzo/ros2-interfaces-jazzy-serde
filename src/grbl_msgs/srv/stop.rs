use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StopRequest {
    pub machine_id: ::std::string::String,
    pub command: ::std::string::String,
}

impl Default for StopRequest {
    fn default() -> Self {
        StopRequest {
            machine_id: ::std::string::String::new(),
            command: ::std::string::String::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for StopRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StopResponse {
    pub success: bool,
}

impl Default for StopResponse {
    fn default() -> Self {
        StopResponse {
            success: false,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for StopResponse {}


pub struct Stop;
