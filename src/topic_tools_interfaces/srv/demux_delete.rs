use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DemuxDeleteRequest {
    pub topic: ::std::string::String,
}

impl Default for DemuxDeleteRequest {
    fn default() -> Self {
        DemuxDeleteRequest {
            topic: ::std::string::String::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for DemuxDeleteRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DemuxDeleteResponse {
    pub success: bool,
}

impl Default for DemuxDeleteResponse {
    fn default() -> Self {
        DemuxDeleteResponse {
            success: false,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for DemuxDeleteResponse {}


pub struct DemuxDelete;
