use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetInitialPoseRequest {
    pub pose: crate::geometry_msgs::msg::PoseWithCovarianceStamped,
}

impl Default for SetInitialPoseRequest {
    fn default() -> Self {
        SetInitialPoseRequest {
            pose: crate::geometry_msgs::msg::PoseWithCovarianceStamped::default(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for SetInitialPoseRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetInitialPoseResponse {

}

impl Default for SetInitialPoseResponse {
    fn default() -> Self {
        SetInitialPoseResponse {

        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for SetInitialPoseResponse {}


pub struct SetInitialPose;
