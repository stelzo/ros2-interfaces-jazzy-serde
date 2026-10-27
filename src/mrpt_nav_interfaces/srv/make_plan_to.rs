use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MakePlanToRequest {
    pub target: crate::geometry_msgs::msg::PoseStamped,
}

impl Default for MakePlanToRequest {
    fn default() -> Self {
        MakePlanToRequest {
            target: crate::geometry_msgs::msg::PoseStamped::default(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for MakePlanToRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MakePlanToResponse {
    pub valid_path_found: bool,
    pub waypoints: crate::mrpt_msgs::msg::WaypointSequence,
}

impl Default for MakePlanToResponse {
    fn default() -> Self {
        MakePlanToResponse {
            valid_path_found: false,
            waypoints: crate::mrpt_msgs::msg::WaypointSequence::default(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for MakePlanToResponse {}


pub struct MakePlanTo;
