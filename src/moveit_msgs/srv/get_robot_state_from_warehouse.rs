use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetRobotStateFromWarehouseRequest {
    pub name: ::std::string::String,
    pub robot: ::std::string::String,
}

impl Default for GetRobotStateFromWarehouseRequest {
    fn default() -> Self {
        GetRobotStateFromWarehouseRequest {
            name: ::std::string::String::new(),
            robot: ::std::string::String::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for GetRobotStateFromWarehouseRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetRobotStateFromWarehouseResponse {
    pub state: crate::moveit_msgs::msg::RobotState,
}

impl Default for GetRobotStateFromWarehouseResponse {
    fn default() -> Self {
        GetRobotStateFromWarehouseResponse {
            state: crate::moveit_msgs::msg::RobotState::default(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for GetRobotStateFromWarehouseResponse {}


pub struct GetRobotStateFromWarehouse;
