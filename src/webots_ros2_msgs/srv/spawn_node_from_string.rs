use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpawnNodeFromStringRequest {
    pub data: ::std::string::String,
}

impl Default for SpawnNodeFromStringRequest {
    fn default() -> Self {
        SpawnNodeFromStringRequest {
            data: ::std::string::String::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for SpawnNodeFromStringRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpawnNodeFromStringResponse {
    pub success: bool,
}

impl Default for SpawnNodeFromStringResponse {
    fn default() -> Self {
        SpawnNodeFromStringResponse {
            success: false,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for SpawnNodeFromStringResponse {}


pub struct SpawnNodeFromString;
