use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CancelTriggerRequest {
    pub name: ::std::string::String,
}

impl Default for CancelTriggerRequest {
    fn default() -> Self {
        CancelTriggerRequest {
            name: ::std::string::String::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for CancelTriggerRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CancelTriggerResponse {
    pub success: bool,
    pub message: ::std::string::String,
}

impl Default for CancelTriggerResponse {
    fn default() -> Self {
        CancelTriggerResponse {
            success: false,
            message: ::std::string::String::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for CancelTriggerResponse {}


pub struct CancelTrigger;
