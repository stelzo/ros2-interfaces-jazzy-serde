use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommandVtolTransitionRequest {
    pub header: crate::std_msgs::msg::Header,
    pub state: u8,
}

impl CommandVtolTransitionRequest {
    pub const STATE_MC: u8 = 3;
    pub const STATE_FW: u8 = 4;
}

impl Default for CommandVtolTransitionRequest {
    fn default() -> Self {
        CommandVtolTransitionRequest {
            header: crate::std_msgs::msg::Header::default(),
            state: 0,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for CommandVtolTransitionRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommandVtolTransitionResponse {
    pub success: bool,
    pub result: u8,
}

impl Default for CommandVtolTransitionResponse {
    fn default() -> Self {
        CommandVtolTransitionResponse {
            success: false,
            result: 0,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for CommandVtolTransitionResponse {}


pub struct CommandVtolTransition;
