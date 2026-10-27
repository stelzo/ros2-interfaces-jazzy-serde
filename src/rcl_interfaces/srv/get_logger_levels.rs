use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetLoggerLevelsRequest {
    pub names: Vec<::std::string::String>,
}

impl Default for GetLoggerLevelsRequest {
    fn default() -> Self {
        GetLoggerLevelsRequest {
            names: Vec::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for GetLoggerLevelsRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetLoggerLevelsResponse {
    pub levels: Vec<crate::rcl_interfaces::msg::LoggerLevel>,
}

impl Default for GetLoggerLevelsResponse {
    fn default() -> Self {
        GetLoggerLevelsResponse {
            levels: Vec::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for GetLoggerLevelsResponse {}


pub struct GetLoggerLevels;
