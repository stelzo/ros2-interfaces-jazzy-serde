use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WarmStartRequest {
    pub reset_type: u8,
}

impl Default for WarmStartRequest {
    fn default() -> Self {
        WarmStartRequest {
            reset_type: 0,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for WarmStartRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WarmStartResponse {

}

impl Default for WarmStartResponse {
    fn default() -> Self {
        WarmStartResponse {

        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for WarmStartResponse {}


pub struct WarmStart;
