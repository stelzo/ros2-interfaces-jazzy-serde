use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IntrospectSubscribersRequest {

}

impl Default for IntrospectSubscribersRequest {
    fn default() -> Self {
        IntrospectSubscribersRequest {

        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for IntrospectSubscribersRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IntrospectSubscribersResponse {
    pub subscriber_details: Vec<crate::py_trees_ros_interfaces::msg::SubscriberDetails>,
}

impl Default for IntrospectSubscribersResponse {
    fn default() -> Self {
        IntrospectSubscribersResponse {
            subscriber_details: Vec::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for IntrospectSubscribersResponse {}


pub struct IntrospectSubscribers;
