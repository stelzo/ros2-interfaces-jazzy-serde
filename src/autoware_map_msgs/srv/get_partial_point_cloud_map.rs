use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetPartialPointCloudMapRequest {
    pub area: crate::autoware_map_msgs::msg::AreaInfo,
}

impl Default for GetPartialPointCloudMapRequest {
    fn default() -> Self {
        GetPartialPointCloudMapRequest {
            area: crate::autoware_map_msgs::msg::AreaInfo::default(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for GetPartialPointCloudMapRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetPartialPointCloudMapResponse {
    pub header: crate::std_msgs::msg::Header,
    pub new_pointcloud_with_ids: Vec<crate::autoware_map_msgs::msg::PointCloudMapCellWithID>,
}

impl Default for GetPartialPointCloudMapResponse {
    fn default() -> Self {
        GetPartialPointCloudMapResponse {
            header: crate::std_msgs::msg::Header::default(),
            new_pointcloud_with_ids: Vec::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for GetPartialPointCloudMapResponse {}


pub struct GetPartialPointCloudMap;
