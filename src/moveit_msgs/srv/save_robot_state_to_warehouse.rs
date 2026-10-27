use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SaveRobotStateToWarehouseRequest {
    pub name: ::std::string::String,
    pub robot: ::std::string::String,
    pub state: crate::moveit_msgs::msg::RobotState,
}

impl Default for SaveRobotStateToWarehouseRequest {
    fn default() -> Self {
        SaveRobotStateToWarehouseRequest {
            name: ::std::string::String::new(),
            robot: ::std::string::String::new(),
            state: crate::moveit_msgs::msg::RobotState::default(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for SaveRobotStateToWarehouseRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SaveRobotStateToWarehouseResponse {
    pub success: bool,
}

impl Default for SaveRobotStateToWarehouseResponse {
    fn default() -> Self {
        SaveRobotStateToWarehouseResponse {
            success: false,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for SaveRobotStateToWarehouseResponse {}


pub struct SaveRobotStateToWarehouse;
