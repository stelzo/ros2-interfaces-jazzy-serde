use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetStringRequest {
    pub value: ::std::string::String,
}

impl Default for SetStringRequest {
    fn default() -> Self {
        SetStringRequest {
            value: ::std::string::String::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for SetStringRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetStringResponse {
    pub success: bool,
}

impl Default for SetStringResponse {
    fn default() -> Self {
        SetStringResponse {
            success: false,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for SetStringResponse {}


pub struct SetString;
