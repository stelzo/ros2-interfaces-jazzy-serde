use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServiceNodeRequest {
    pub service: ::std::string::String,
}

impl Default for ServiceNodeRequest {
    fn default() -> Self {
        ServiceNodeRequest {
            service: ::std::string::String::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for ServiceNodeRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServiceNodeResponse {
    pub node: ::std::string::String,
}

impl Default for ServiceNodeResponse {
    fn default() -> Self {
        ServiceNodeResponse {
            node: ::std::string::String::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for ServiceNodeResponse {}


pub struct ServiceNode;
