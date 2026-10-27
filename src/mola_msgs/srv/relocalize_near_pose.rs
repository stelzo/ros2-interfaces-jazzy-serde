use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RelocalizeNearPoseRequest {
    pub pose: crate::geometry_msgs::msg::PoseWithCovarianceStamped,
}

impl Default for RelocalizeNearPoseRequest {
    fn default() -> Self {
        RelocalizeNearPoseRequest {
            pose: crate::geometry_msgs::msg::PoseWithCovarianceStamped::default(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for RelocalizeNearPoseRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RelocalizeNearPoseResponse {
    pub accepted: bool,
}

impl Default for RelocalizeNearPoseResponse {
    fn default() -> Self {
        RelocalizeNearPoseResponse {
            accepted: false,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for RelocalizeNearPoseResponse {}


pub struct RelocalizeNearPose;
