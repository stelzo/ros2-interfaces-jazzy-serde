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
    pub return_code: crate::rc_common_msgs::msg::ReturnCode,
}

impl Default for TriggerResponse {
    fn default() -> Self {
        TriggerResponse {
            return_code: crate::rc_common_msgs::msg::ReturnCode::default(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for TriggerResponse {}


pub struct Trigger;
