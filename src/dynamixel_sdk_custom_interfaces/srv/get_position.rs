use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetPositionRequest {
    pub id: u8,
}

impl Default for GetPositionRequest {
    fn default() -> Self {
        GetPositionRequest {
            id: 0,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for GetPositionRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetPositionResponse {
    pub position: i32,
}

impl Default for GetPositionResponse {
    fn default() -> Self {
        GetPositionResponse {
            position: 0,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for GetPositionResponse {}


pub struct GetPosition;
#[cfg(feature = "ros2-client")]
impl ros2_client::Service for GetPosition {
    type Request = GetPositionRequest;
    type Response = GetPositionResponse;

    fn request_type_name(&self) -> &str { "GetPositionRequest" }
    fn response_type_name(&self) -> &str { "GetPositionResponse" }
}
