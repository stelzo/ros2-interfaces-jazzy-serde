use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TriggerRequest {

}

impl Default for TriggerRequest {
    fn default() -> Self {
        TriggerRequest {

        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for TriggerRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TriggerResponse {
    pub success: bool,
    pub message: ::std::string::String,
}

impl Default for TriggerResponse {
    fn default() -> Self {
        TriggerResponse {
            success: false,
            message: ::std::string::String::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for TriggerResponse {}


pub struct Trigger;
