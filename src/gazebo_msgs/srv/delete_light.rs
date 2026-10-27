use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteLightRequest {
    pub light_name: ::std::string::String,
}

impl Default for DeleteLightRequest {
    fn default() -> Self {
        DeleteLightRequest {
            light_name: ::std::string::String::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for DeleteLightRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteLightResponse {
    pub success: bool,
    pub status_message: ::std::string::String,
}

impl Default for DeleteLightResponse {
    fn default() -> Self {
        DeleteLightResponse {
            success: false,
            status_message: ::std::string::String::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for DeleteLightResponse {}


pub struct DeleteLight;
