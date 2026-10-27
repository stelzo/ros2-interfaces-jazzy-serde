use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnloadNodeRequest {
    pub unique_id: u64,
}

impl Default for UnloadNodeRequest {
    fn default() -> Self {
        UnloadNodeRequest {
            unique_id: 0,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for UnloadNodeRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnloadNodeResponse {
    pub success: bool,
    pub error_message: ::std::string::String,
}

impl Default for UnloadNodeResponse {
    fn default() -> Self {
        UnloadNodeResponse {
            success: false,
            error_message: ::std::string::String::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for UnloadNodeResponse {}


pub struct UnloadNode;
