use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServicesRequest {

}

impl Default for ServicesRequest {
    fn default() -> Self {
        ServicesRequest {

        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for ServicesRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServicesResponse {
    pub services: Vec<::std::string::String>,
}

impl Default for ServicesResponse {
    fn default() -> Self {
        ServicesResponse {
            services: Vec::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for ServicesResponse {}


pub struct Services;
