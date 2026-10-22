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
#[cfg(feature = "ros2-client")]
impl ros2_client::Service for DeleteRobotStateFromWarehouse {
    type Request = DeleteRobotStateFromWarehouseRequest;
    type Response = DeleteRobotStateFromWarehouseResponse;

    fn request_type_name(&self) -> &str { "DeleteRobotStateFromWarehouseRequest" }
    fn response_type_name(&self) -> &str { "DeleteRobotStateFromWarehouseResponse" }
}
