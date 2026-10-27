use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MuxListRequest {

}

impl Default for MuxListRequest {
    fn default() -> Self {
        MuxListRequest {

        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for MuxListRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MuxListResponse {
    pub topics: Vec<::std::string::String>,
}

impl Default for MuxListResponse {
    fn default() -> Self {
        MuxListResponse {
            topics: Vec::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for MuxListResponse {}


pub struct MuxList;
