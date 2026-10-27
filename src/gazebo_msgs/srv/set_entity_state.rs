use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetEntityStateRequest {
    pub state: crate::gazebo_msgs::msg::EntityState,
}

impl Default for SetEntityStateRequest {
    fn default() -> Self {
        SetEntityStateRequest {
            state: crate::gazebo_msgs::msg::EntityState::default(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for SetEntityStateRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetEntityStateResponse {
    pub success: bool,
}

impl Default for SetEntityStateResponse {
    fn default() -> Self {
        SetEntityStateResponse {
            success: false,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for SetEntityStateResponse {}


pub struct SetEntityState;
