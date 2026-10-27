use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServicesForTypeRequest {
    #[serde(rename = "type")]    pub type_: ::std::string::String,
}

impl Default for ServicesForTypeRequest {
    fn default() -> Self {
        ServicesForTypeRequest {
            type_: ::std::string::String::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for ServicesForTypeRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServicesForTypeResponse {
    pub services: Vec<::std::string::String>,
}

impl Default for ServicesForTypeResponse {
    fn default() -> Self {
        ServicesForTypeResponse {
            services: Vec::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for ServicesForTypeResponse {}


pub struct ServicesForType;
