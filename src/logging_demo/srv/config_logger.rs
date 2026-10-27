use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConfigLoggerRequest {
    pub logger_name: ::std::string::String,
    pub level: ::std::string::String,
}

impl Default for ConfigLoggerRequest {
    fn default() -> Self {
        ConfigLoggerRequest {
            logger_name: ::std::string::String::new(),
            level: ::std::string::String::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for ConfigLoggerRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConfigLoggerResponse {
    pub success: bool,
}

impl Default for ConfigLoggerResponse {
    fn default() -> Self {
        ConfigLoggerResponse {
            success: false,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for ConfigLoggerResponse {}


pub struct ConfigLogger;
