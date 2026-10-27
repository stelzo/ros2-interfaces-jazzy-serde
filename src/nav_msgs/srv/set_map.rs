use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetMapRequest {
    pub map: crate::nav_msgs::msg::OccupancyGrid,
    pub initial_pose: crate::geometry_msgs::msg::PoseWithCovarianceStamped,
}

impl Default for SetMapRequest {
    fn default() -> Self {
        SetMapRequest {
            map: crate::nav_msgs::msg::OccupancyGrid::default(),
            initial_pose: crate::geometry_msgs::msg::PoseWithCovarianceStamped::default(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for SetMapRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetMapResponse {
    pub success: bool,
}

impl Default for SetMapResponse {
    fn default() -> Self {
        SetMapResponse {
            success: false,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for SetMapResponse {}


pub struct SetMap;
