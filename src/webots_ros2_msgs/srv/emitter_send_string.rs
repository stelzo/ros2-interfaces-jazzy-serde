use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmitterSendStringRequest {
    pub value: ::std::string::String,
}

impl Default for EmitterSendStringRequest {
    fn default() -> Self {
        EmitterSendStringRequest {
            value: ::std::string::String::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for EmitterSendStringRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmitterSendStringResponse {
    pub result: i64,
}

impl Default for EmitterSendStringResponse {
    fn default() -> Self {
        EmitterSendStringResponse {
            result: 0,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for EmitterSendStringResponse {}


pub struct EmitterSendString;
#[cfg(feature = "ros2-client")]
impl ros2_client::Service for EmitterSendString {
    type Request = EmitterSendStringRequest;
    type Response = EmitterSendStringResponse;

    fn request_type_name(&self) -> &str { "EmitterSendStringRequest" }
    fn response_type_name(&self) -> &str { "EmitterSendStringResponse" }
}
