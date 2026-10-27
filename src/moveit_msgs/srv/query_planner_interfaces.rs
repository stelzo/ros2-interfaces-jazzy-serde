use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QueryPlannerInterfacesRequest {

}

impl Default for QueryPlannerInterfacesRequest {
    fn default() -> Self {
        QueryPlannerInterfacesRequest {

        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for QueryPlannerInterfacesRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QueryPlannerInterfacesResponse {
    pub planner_interfaces: Vec<crate::moveit_msgs::msg::PlannerInterfaceDescription>,
}

impl Default for QueryPlannerInterfacesResponse {
    fn default() -> Self {
        QueryPlannerInterfacesResponse {
            planner_interfaces: Vec::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for QueryPlannerInterfacesResponse {}


pub struct QueryPlannerInterfaces;
