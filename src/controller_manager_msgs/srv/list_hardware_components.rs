use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListHardwareComponentsRequest {

}

impl Default for ListHardwareComponentsRequest {
    fn default() -> Self {
        ListHardwareComponentsRequest {

        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for ListHardwareComponentsRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListHardwareComponentsResponse {
    pub component: Vec<crate::controller_manager_msgs::msg::HardwareComponentState>,
}

impl Default for ListHardwareComponentsResponse {
    fn default() -> Self {
        ListHardwareComponentsResponse {
            component: Vec::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for ListHardwareComponentsResponse {}


pub struct ListHardwareComponents;
