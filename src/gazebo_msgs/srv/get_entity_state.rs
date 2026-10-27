use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetEntityStateRequest {
    pub name: ::std::string::String,
    pub reference_frame: ::std::string::String,
}

impl Default for GetEntityStateRequest {
    fn default() -> Self {
        GetEntityStateRequest {
            name: ::std::string::String::new(),
            reference_frame: ::std::string::String::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for GetEntityStateRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetEntityStateResponse {
    pub header: crate::std_msgs::msg::Header,
    pub state: crate::gazebo_msgs::msg::EntityState,
    pub success: bool,
}

impl Default for GetEntityStateResponse {
    fn default() -> Self {
        GetEntityStateResponse {
            header: crate::std_msgs::msg::Header::default(),
            state: crate::gazebo_msgs::msg::EntityState::default(),
            success: false,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for GetEntityStateResponse {}


pub struct GetEntityState;
