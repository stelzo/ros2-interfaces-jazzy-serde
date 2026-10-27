use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubscribersRequest {
    pub topic: ::std::string::String,
}

impl Default for SubscribersRequest {
    fn default() -> Self {
        SubscribersRequest {
            topic: ::std::string::String::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for SubscribersRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubscribersResponse {
    pub subscribers: Vec<::std::string::String>,
}

impl Default for SubscribersResponse {
    fn default() -> Self {
        SubscribersResponse {
            subscribers: Vec::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for SubscribersResponse {}


pub struct Subscribers;
