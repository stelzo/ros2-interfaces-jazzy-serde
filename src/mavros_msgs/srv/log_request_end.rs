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
#[cfg(feature = "ros2-client")]
impl ros2_client::Service for LogRequestEnd {
    type Request = LogRequestEndRequest;
    type Response = LogRequestEndResponse;

    fn request_type_name(&self) -> &str { "LogRequestEndRequest" }
    fn response_type_name(&self) -> &str { "LogRequestEndResponse" }
}
