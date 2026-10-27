use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DemuxListRequest {

}

impl Default for DemuxListRequest {
    fn default() -> Self {
        DemuxListRequest {

        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for DemuxListRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DemuxListResponse {
    pub topics: Vec<::std::string::String>,
}

impl Default for DemuxListResponse {
    fn default() -> Self {
        DemuxListResponse {
            topics: Vec::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for DemuxListResponse {}


pub struct DemuxList;
