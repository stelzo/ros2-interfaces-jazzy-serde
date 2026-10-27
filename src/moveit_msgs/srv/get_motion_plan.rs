use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMotionPlanRequest {
    pub motion_plan_request: crate::moveit_msgs::msg::MotionPlanRequest,
}

impl Default for GetMotionPlanRequest {
    fn default() -> Self {
        GetMotionPlanRequest {
            motion_plan_request: crate::moveit_msgs::msg::MotionPlanRequest::default(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for GetMotionPlanRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMotionPlanResponse {
    pub motion_plan_response: crate::moveit_msgs::msg::MotionPlanResponse,
}

impl Default for GetMotionPlanResponse {
    fn default() -> Self {
        GetMotionPlanResponse {
            motion_plan_response: crate::moveit_msgs::msg::MotionPlanResponse::default(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for GetMotionPlanResponse {}


pub struct GetMotionPlan;
