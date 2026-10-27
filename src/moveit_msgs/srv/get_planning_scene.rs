use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetPlanningSceneRequest {
    pub components: crate::moveit_msgs::msg::PlanningSceneComponents,
}

impl Default for GetPlanningSceneRequest {
    fn default() -> Self {
        GetPlanningSceneRequest {
            components: crate::moveit_msgs::msg::PlanningSceneComponents::default(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for GetPlanningSceneRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetPlanningSceneResponse {
    pub scene: crate::moveit_msgs::msg::PlanningScene,
}

impl Default for GetPlanningSceneResponse {
    fn default() -> Self {
        GetPlanningSceneResponse {
            scene: crate::moveit_msgs::msg::PlanningScene::default(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for GetPlanningSceneResponse {}


pub struct GetPlanningScene;
