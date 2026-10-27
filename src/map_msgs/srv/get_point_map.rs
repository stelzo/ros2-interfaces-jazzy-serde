use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetPointMapRequest {

}

impl Default for GetPointMapRequest {
    fn default() -> Self {
        GetPointMapRequest {

        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for GetPointMapRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetPointMapResponse {
    pub map: crate::sensor_msgs::msg::PointCloud2,
}

impl Default for GetPointMapResponse {
    fn default() -> Self {
        GetPointMapResponse {
            map: crate::sensor_msgs::msg::PointCloud2::default(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for GetPointMapResponse {}


pub struct GetPointMap;
