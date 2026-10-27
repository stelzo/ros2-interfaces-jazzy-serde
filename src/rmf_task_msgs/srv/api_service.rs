use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApiServiceRequest {
    pub json_msg: ::std::string::String,
}

impl Default for ApiServiceRequest {
    fn default() -> Self {
        ApiServiceRequest {
            json_msg: ::std::string::String::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for ApiServiceRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApiServiceResponse {
    pub json_msg: ::std::string::String,
}

impl Default for ApiServiceResponse {
    fn default() -> Self {
        ApiServiceResponse {
            json_msg: ::std::string::String::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for ApiServiceResponse {}


pub struct ApiService;
