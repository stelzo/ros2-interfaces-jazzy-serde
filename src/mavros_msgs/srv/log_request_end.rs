use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LogRequestEndRequest {

}

impl Default for LogRequestEndRequest {
    fn default() -> Self {
        LogRequestEndRequest {

        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for LogRequestEndRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LogRequestEndResponse {
    pub success: bool,
}

impl Default for LogRequestEndResponse {
    fn default() -> Self {
        LogRequestEndResponse {
            success: false,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for LogRequestEndResponse {}


pub struct LogRequestEnd;
