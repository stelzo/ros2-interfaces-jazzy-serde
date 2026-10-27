use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChangeStateRequest {
    pub transition: crate::lifecycle_msgs::msg::Transition,
}

impl Default for ChangeStateRequest {
    fn default() -> Self {
        ChangeStateRequest {
            transition: crate::lifecycle_msgs::msg::Transition::default(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for ChangeStateRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChangeStateResponse {
    pub success: bool,
}

impl Default for ChangeStateResponse {
    fn default() -> Self {
        ChangeStateResponse {
            success: false,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for ChangeStateResponse {}


pub struct ChangeState;
