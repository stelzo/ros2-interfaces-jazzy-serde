use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApplyPlanningSceneRequest {
    pub scene: crate::moveit_msgs::msg::PlanningScene,
}

impl Default for ApplyPlanningSceneRequest {
    fn default() -> Self {
        ApplyPlanningSceneRequest {
            scene: crate::moveit_msgs::msg::PlanningScene::default(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for ApplyPlanningSceneRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApplyPlanningSceneResponse {
    pub success: bool,
}

impl Default for ApplyPlanningSceneResponse {
    fn default() -> Self {
        ApplyPlanningSceneResponse {
            success: false,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for ApplyPlanningSceneResponse {}


pub struct ApplyPlanningScene;
