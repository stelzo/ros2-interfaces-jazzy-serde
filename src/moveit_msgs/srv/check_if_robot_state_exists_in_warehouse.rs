use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CheckIfRobotStateExistsInWarehouseRequest {
    pub name: ::std::string::String,
    pub robot: ::std::string::String,
}

impl Default for CheckIfRobotStateExistsInWarehouseRequest {
    fn default() -> Self {
        CheckIfRobotStateExistsInWarehouseRequest {
            name: ::std::string::String::new(),
            robot: ::std::string::String::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for CheckIfRobotStateExistsInWarehouseRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CheckIfRobotStateExistsInWarehouseResponse {
    pub exists: bool,
}

impl Default for CheckIfRobotStateExistsInWarehouseResponse {
    fn default() -> Self {
        CheckIfRobotStateExistsInWarehouseResponse {
            exists: false,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for CheckIfRobotStateExistsInWarehouseResponse {}


pub struct CheckIfRobotStateExistsInWarehouse;
