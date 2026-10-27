use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TriggerNamedRequest {
    pub name: ::std::string::String,
}

impl Default for TriggerNamedRequest {
    fn default() -> Self {
        TriggerNamedRequest {
            name: ::std::string::String::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for TriggerNamedRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TriggerNamedResponse {
    pub success: bool,
    pub message: ::std::string::String,
}

impl Default for TriggerNamedResponse {
    fn default() -> Self {
        TriggerNamedResponse {
            success: false,
            message: ::std::string::String::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for TriggerNamedResponse {}


pub struct TriggerNamed;
