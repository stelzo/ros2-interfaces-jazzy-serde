use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommandBoolRequest {
    pub value: bool,
}

impl Default for CommandBoolRequest {
    fn default() -> Self {
        CommandBoolRequest {
            value: false,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for CommandBoolRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommandBoolResponse {
    pub success: bool,
    pub result: u8,
}

impl Default for CommandBoolResponse {
    fn default() -> Self {
        CommandBoolResponse {
            success: false,
            result: 0,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for CommandBoolResponse {}


pub struct CommandBool;
