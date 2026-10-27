use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdatePointcloudOctomapRequest {
    pub cloud: crate::sensor_msgs::msg::PointCloud2,
}

impl Default for UpdatePointcloudOctomapRequest {
    fn default() -> Self {
        UpdatePointcloudOctomapRequest {
            cloud: crate::sensor_msgs::msg::PointCloud2::default(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for UpdatePointcloudOctomapRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdatePointcloudOctomapResponse {
    pub success: bool,
}

impl Default for UpdatePointcloudOctomapResponse {
    fn default() -> Self {
        UpdatePointcloudOctomapResponse {
            success: false,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for UpdatePointcloudOctomapResponse {}


pub struct UpdatePointcloudOctomap;
