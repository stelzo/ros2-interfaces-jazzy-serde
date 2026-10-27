use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetLoggerLevelsRequest {
    pub levels: Vec<crate::rcl_interfaces::msg::LoggerLevel>,
}

impl Default for SetLoggerLevelsRequest {
    fn default() -> Self {
        SetLoggerLevelsRequest {
            levels: Vec::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for SetLoggerLevelsRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetLoggerLevelsResponse {
    pub results: Vec<crate::rcl_interfaces::msg::SetLoggerLevelsResult>,
}

impl Default for SetLoggerLevelsResponse {
    fn default() -> Self {
        SetLoggerLevelsResponse {
            results: Vec::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for SetLoggerLevelsResponse {}


pub struct SetLoggerLevels;
