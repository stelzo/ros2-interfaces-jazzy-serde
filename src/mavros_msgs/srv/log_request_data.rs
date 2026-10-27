use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LogRequestDataRequest {
    pub id: u16,
    pub offset: u32,
    pub count: u32,
}

impl Default for LogRequestDataRequest {
    fn default() -> Self {
        LogRequestDataRequest {
            id: 0,
            offset: 0,
            count: 0,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for LogRequestDataRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LogRequestDataResponse {
    pub success: bool,
}

impl Default for LogRequestDataResponse {
    fn default() -> Self {
        LogRequestDataResponse {
            success: false,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for LogRequestDataResponse {}


pub struct LogRequestData;
