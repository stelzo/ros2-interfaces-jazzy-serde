use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMotionSequenceRequest {
    pub request: crate::moveit_msgs::msg::MotionSequenceRequest,
}

impl Default for GetMotionSequenceRequest {
    fn default() -> Self {
        GetMotionSequenceRequest {
            request: crate::moveit_msgs::msg::MotionSequenceRequest::default(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for GetMotionSequenceRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMotionSequenceResponse {
    pub response: crate::moveit_msgs::msg::MotionSequenceResponse,
}

impl Default for GetMotionSequenceResponse {
    fn default() -> Self {
        GetMotionSequenceResponse {
            response: crate::moveit_msgs::msg::MotionSequenceResponse::default(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for GetMotionSequenceResponse {}


pub struct GetMotionSequence;
