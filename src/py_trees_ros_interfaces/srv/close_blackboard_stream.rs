use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CloseBlackboardStreamRequest {
    pub topic_name: ::std::string::String,
}

impl Default for CloseBlackboardStreamRequest {
    fn default() -> Self {
        CloseBlackboardStreamRequest {
            topic_name: ::std::string::String::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for CloseBlackboardStreamRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CloseBlackboardStreamResponse {
    pub result: bool,
}

impl Default for CloseBlackboardStreamResponse {
    fn default() -> Self {
        CloseBlackboardStreamResponse {
            result: false,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for CloseBlackboardStreamResponse {}


pub struct CloseBlackboardStream;
