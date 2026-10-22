use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetLinkStateRequest {
    pub link_state: crate::gazebo_msgs::msg::LinkState,
}

impl Default for SetLinkStateRequest {
    fn default() -> Self {
        SetLinkStateRequest {
            link_state: crate::gazebo_msgs::msg::LinkState::default(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for SetLinkStateRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetLinkStateResponse {
    pub success: bool,
    pub status_message: ::std::string::String,
}

impl Default for SetLinkStateResponse {
    fn default() -> Self {
        SetLinkStateResponse {
            success: false,
            status_message: ::std::string::String::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for SetLinkStateResponse {}


pub struct SetLinkState;
#[cfg(feature = "ros2-client")]
impl ros2_client::Service for SetLinkState {
    type Request = SetLinkStateRequest;
    type Response = SetLinkStateResponse;

    fn request_type_name(&self) -> &str { "SetLinkStateRequest" }
    fn response_type_name(&self) -> &str { "SetLinkStateResponse" }
}
