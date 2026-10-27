use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FinishTrajectoryRequest {
    pub trajectory_id: i32,
}

impl Default for FinishTrajectoryRequest {
    fn default() -> Self {
        FinishTrajectoryRequest {
            trajectory_id: 0,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for FinishTrajectoryRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FinishTrajectoryResponse {
    pub status: crate::cartographer_ros_msgs::msg::StatusResponse,
}

impl Default for FinishTrajectoryResponse {
    fn default() -> Self {
        FinishTrajectoryResponse {
            status: crate::cartographer_ros_msgs::msg::StatusResponse::default(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for FinishTrajectoryResponse {}


pub struct FinishTrajectory;
