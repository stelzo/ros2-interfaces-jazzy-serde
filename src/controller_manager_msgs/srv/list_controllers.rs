use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListControllersRequest {

}

impl Default for ListControllersRequest {
    fn default() -> Self {
        ListControllersRequest {

        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for ListControllersRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListControllersResponse {
    pub controller: Vec<crate::controller_manager_msgs::msg::ControllerState>,
}

impl Default for ListControllersResponse {
    fn default() -> Self {
        ListControllersResponse {
            controller: Vec::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for ListControllersResponse {}


pub struct ListControllers;
