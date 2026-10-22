use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListHardwareInterfacesRequest {

}

impl Default for ListHardwareInterfacesRequest {
    fn default() -> Self {
        ListHardwareInterfacesRequest {

        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for ListHardwareInterfacesRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListHardwareInterfacesResponse {
    pub command_interfaces: Vec<crate::controller_manager_msgs::msg::HardwareInterface>,
    pub state_interfaces: Vec<crate::controller_manager_msgs::msg::HardwareInterface>,
}

impl Default for ListHardwareInterfacesResponse {
    fn default() -> Self {
        ListHardwareInterfacesResponse {
            command_interfaces: Vec::new(),
            state_interfaces: Vec::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for ListHardwareInterfacesResponse {}


pub struct ListHardwareInterfaces;
#[cfg(feature = "ros2-client")]
impl ros2_client::Service for ListHardwareInterfaces {
    type Request = ListHardwareInterfacesRequest;
    type Response = ListHardwareInterfacesResponse;

    fn request_type_name(&self) -> &str { "ListHardwareInterfacesRequest" }
    fn response_type_name(&self) -> &str { "ListHardwareInterfacesResponse" }
}
