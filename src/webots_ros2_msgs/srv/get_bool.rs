use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetBoolRequest {
    pub ask: bool,
}

impl Default for GetBoolRequest {
    fn default() -> Self {
        GetBoolRequest {
            ask: false,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for GetBoolRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetBoolResponse {
    pub value: bool,
}

impl Default for GetBoolResponse {
    fn default() -> Self {
        GetBoolResponse {
            value: false,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for GetBoolResponse {}


pub struct GetBool;
