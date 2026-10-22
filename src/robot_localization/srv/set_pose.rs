use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetPoseRequest {
    pub pose: crate::geometry_msgs::msg::PoseWithCovarianceStamped,
}

impl Default for SetPoseRequest {
    fn default() -> Self {
        SetPoseRequest {
            pose: crate::geometry_msgs::msg::PoseWithCovarianceStamped::default(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for SetPoseRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetPoseResponse {

}

impl Default for SetPoseResponse {
    fn default() -> Self {
        SetPoseResponse {

        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for SetPoseResponse {}


pub struct SetPose;
#[cfg(feature = "ros2-client")]
impl ros2_client::Service for SetPose {
    type Request = SetPoseRequest;
    type Response = SetPoseResponse;

    fn request_type_name(&self) -> &str { "SetPoseRequest" }
    fn response_type_name(&self) -> &str { "SetPoseResponse" }
}
