use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServiceProvidersRequest {
    pub service: ::std::string::String,
}

impl Default for ServiceProvidersRequest {
    fn default() -> Self {
        ServiceProvidersRequest {
            service: ::std::string::String::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for ServiceProvidersRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServiceProvidersResponse {
    pub providers: Vec<::std::string::String>,
}

impl Default for ServiceProvidersResponse {
    fn default() -> Self {
        ServiceProvidersResponse {
            providers: Vec::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for ServiceProvidersResponse {}


pub struct ServiceProviders;
