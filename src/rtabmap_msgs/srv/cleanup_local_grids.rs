use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CleanupLocalGridsRequest {
    pub radius: i32,
    pub filter_scans: bool,
}

impl Default for CleanupLocalGridsRequest {
    fn default() -> Self {
        CleanupLocalGridsRequest {
            radius: 0,
            filter_scans: false,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for CleanupLocalGridsRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CleanupLocalGridsResponse {
    pub modified: i32,
}

impl Default for CleanupLocalGridsResponse {
    fn default() -> Self {
        CleanupLocalGridsResponse {
            modified: 0,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for CleanupLocalGridsResponse {}


pub struct CleanupLocalGrids;
