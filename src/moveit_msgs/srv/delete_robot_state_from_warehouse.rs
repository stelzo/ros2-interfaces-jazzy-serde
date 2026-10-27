use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteRobotStateFromWarehouseRequest {
    pub name: ::std::string::String,
    pub robot: ::std::string::String,
}

impl Default for DeleteRobotStateFromWarehouseRequest {
    fn default() -> Self {
        DeleteRobotStateFromWarehouseRequest {
            name: ::std::string::String::new(),
            robot: ::std::string::String::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for DeleteRobotStateFromWarehouseRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteRobotStateFromWarehouseResponse {

}

impl Default for DeleteRobotStateFromWarehouseResponse {
    fn default() -> Self {
        DeleteRobotStateFromWarehouseResponse {

        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for DeleteRobotStateFromWarehouseResponse {}


pub struct DeleteRobotStateFromWarehouse;
