use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RawRequestRequest {
    pub query: ::std::string::String,
}

impl Default for RawRequestRequest {
    fn default() -> Self {
        RawRequestRequest {
            query: ::std::string::String::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for RawRequestRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RawRequestResponse {
    pub answer: ::std::string::String,
}

impl Default for RawRequestResponse {
    fn default() -> Self {
        RawRequestResponse {
            answer: ::std::string::String::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for RawRequestResponse {}


pub struct RawRequest;
