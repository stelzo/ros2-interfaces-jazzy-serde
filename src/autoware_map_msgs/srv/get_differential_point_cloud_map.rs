use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetDifferentialPointCloudMapRequest {
    pub area: crate::autoware_map_msgs::msg::AreaInfo,
    pub cached_ids: Vec<::std::string::String>,
}

impl Default for GetDifferentialPointCloudMapRequest {
    fn default() -> Self {
        GetDifferentialPointCloudMapRequest {
            area: crate::autoware_map_msgs::msg::AreaInfo::default(),
            cached_ids: Vec::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for GetDifferentialPointCloudMapRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetDifferentialPointCloudMapResponse {
    pub header: crate::std_msgs::msg::Header,
    pub new_pointcloud_with_ids: Vec<crate::autoware_map_msgs::msg::PointCloudMapCellWithID>,
    pub ids_to_remove: Vec<::std::string::String>,
}

impl Default for GetDifferentialPointCloudMapResponse {
    fn default() -> Self {
        GetDifferentialPointCloudMapResponse {
            header: crate::std_msgs::msg::Header::default(),
            new_pointcloud_with_ids: Vec::new(),
            ids_to_remove: Vec::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for GetDifferentialPointCloudMapResponse {}


pub struct GetDifferentialPointCloudMap;
