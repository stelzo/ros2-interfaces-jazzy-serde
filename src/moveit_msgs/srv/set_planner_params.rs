use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetPlannerParamsRequest {
    pub pipeline_id: ::std::string::String,
    pub planner_config: ::std::string::String,
    pub group: ::std::string::String,
    pub params: crate::moveit_msgs::msg::PlannerParams,
    pub replace: bool,
}

impl Default for SetPlannerParamsRequest {
    fn default() -> Self {
        SetPlannerParamsRequest {
            pipeline_id: ::std::string::String::new(),
            planner_config: ::std::string::String::new(),
            group: ::std::string::String::new(),
            params: crate::moveit_msgs::msg::PlannerParams::default(),
            replace: false,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for SetPlannerParamsRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetPlannerParamsResponse {

}

impl Default for SetPlannerParamsResponse {
    fn default() -> Self {
        SetPlannerParamsResponse {

        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for SetPlannerParamsResponse {}


pub struct SetPlannerParams;
