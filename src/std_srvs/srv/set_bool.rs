use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetBoolRequest {
    pub data: bool,
}

impl Default for SetBoolRequest {
    fn default() -> Self {
        SetBoolRequest {
            data: false,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for SetBoolRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetBoolResponse {
    pub success: bool,
    pub message: ::std::string::String,
}

impl Default for SetBoolResponse {
    fn default() -> Self {
        SetBoolResponse {
            success: false,
            message: ::std::string::String::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for SetBoolResponse {}


pub struct SetBool;
