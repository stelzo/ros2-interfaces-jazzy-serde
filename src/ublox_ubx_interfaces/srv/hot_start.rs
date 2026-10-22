use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HotStartRequest {
    pub reset_type: u8,
}

impl Default for HotStartRequest {
    fn default() -> Self {
        HotStartRequest {
            reset_type: 0,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for HotStartRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HotStartResponse {

}

impl Default for HotStartResponse {
    fn default() -> Self {
        HotStartResponse {

        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for HotStartResponse {}


pub struct HotStart;
#[cfg(feature = "ros2-client")]
impl ros2_client::Service for HotStart {
    type Request = HotStartRequest;
    type Response = HotStartResponse;

    fn request_type_name(&self) -> &str { "HotStartRequest" }
    fn response_type_name(&self) -> &str { "HotStartResponse" }
}
