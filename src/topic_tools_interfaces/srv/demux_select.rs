use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DemuxSelectRequest {
    pub topic: ::std::string::String,
}

impl Default for DemuxSelectRequest {
    fn default() -> Self {
        DemuxSelectRequest {
            topic: ::std::string::String::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for DemuxSelectRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DemuxSelectResponse {
    pub prev_topic: ::std::string::String,
    pub success: bool,
}

impl Default for DemuxSelectResponse {
    fn default() -> Self {
        DemuxSelectResponse {
            prev_topic: ::std::string::String::new(),
            success: false,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for DemuxSelectResponse {}


pub struct DemuxSelect;
