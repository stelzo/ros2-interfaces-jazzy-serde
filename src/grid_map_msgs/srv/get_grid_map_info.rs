use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetGridMapInfoRequest {

}

impl Default for GetGridMapInfoRequest {
    fn default() -> Self {
        GetGridMapInfoRequest {

        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for GetGridMapInfoRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetGridMapInfoResponse {
    pub info: crate::grid_map_msgs::msg::GridMapInfo,
}

impl Default for GetGridMapInfoResponse {
    fn default() -> Self {
        GetGridMapInfoResponse {
            info: crate::grid_map_msgs::msg::GridMapInfo::default(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for GetGridMapInfoResponse {}


pub struct GetGridMapInfo;
