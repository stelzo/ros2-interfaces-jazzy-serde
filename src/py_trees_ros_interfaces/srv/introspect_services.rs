use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IntrospectServicesRequest {

}

impl Default for IntrospectServicesRequest {
    fn default() -> Self {
        IntrospectServicesRequest {

        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for IntrospectServicesRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IntrospectServicesResponse {
    pub service_details: Vec<crate::py_trees_ros_interfaces::msg::ServiceDetails>,
}

impl Default for IntrospectServicesResponse {
    fn default() -> Self {
        IntrospectServicesResponse {
            service_details: Vec::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for IntrospectServicesResponse {}


pub struct IntrospectServices;
#[cfg(feature = "ros2-client")]
impl ros2_client::Service for IntrospectServices {
    type Request = IntrospectServicesRequest;
    type Response = IntrospectServicesResponse;

    fn request_type_name(&self) -> &str { "IntrospectServicesRequest" }
    fn response_type_name(&self) -> &str { "IntrospectServicesResponse" }
}
