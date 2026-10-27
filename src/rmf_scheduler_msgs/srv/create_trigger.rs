use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateTriggerRequest {
    pub trigger: crate::rmf_scheduler_msgs::msg::Trigger,
}

impl Default for CreateTriggerRequest {
    fn default() -> Self {
        CreateTriggerRequest {
            trigger: crate::rmf_scheduler_msgs::msg::Trigger::default(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for CreateTriggerRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateTriggerResponse {
    pub success: bool,
    pub message: ::std::string::String,
}

impl Default for CreateTriggerResponse {
    fn default() -> Self {
        CreateTriggerResponse {
            success: false,
            message: ::std::string::String::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for CreateTriggerResponse {}


pub struct CreateTrigger;
