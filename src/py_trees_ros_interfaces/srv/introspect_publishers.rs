use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IntrospectPublishersRequest {

}

impl Default for IntrospectPublishersRequest {
    fn default() -> Self {
        IntrospectPublishersRequest {

        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for IntrospectPublishersRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IntrospectPublishersResponse {
    pub publisher_details: Vec<crate::py_trees_ros_interfaces::msg::PublisherDetails>,
}

impl Default for IntrospectPublishersResponse {
    fn default() -> Self {
        IntrospectPublishersResponse {
            publisher_details: Vec::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for IntrospectPublishersResponse {}


pub struct IntrospectPublishers;
#[cfg(feature = "ros2-client")]
impl ros2_client::Service for IntrospectPublishers {
    type Request = IntrospectPublishersRequest;
    type Response = IntrospectPublishersResponse;

    fn request_type_name(&self) -> &str { "IntrospectPublishersRequest" }
    fn response_type_name(&self) -> &str { "IntrospectPublishersResponse" }
}
