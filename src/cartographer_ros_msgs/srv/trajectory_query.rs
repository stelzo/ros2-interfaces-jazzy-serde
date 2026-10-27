use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrajectoryQueryRequest {
    pub trajectory_id: i32,
}

impl Default for TrajectoryQueryRequest {
    fn default() -> Self {
        TrajectoryQueryRequest {
            trajectory_id: 0,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for TrajectoryQueryRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrajectoryQueryResponse {
    pub status: crate::cartographer_ros_msgs::msg::StatusResponse,
    pub trajectory: Vec<crate::geometry_msgs::msg::PoseStamped>,
}

impl Default for TrajectoryQueryResponse {
    fn default() -> Self {
        TrajectoryQueryResponse {
            status: crate::cartographer_ros_msgs::msg::StatusResponse::default(),
            trajectory: Vec::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for TrajectoryQueryResponse {}


pub struct TrajectoryQuery;
