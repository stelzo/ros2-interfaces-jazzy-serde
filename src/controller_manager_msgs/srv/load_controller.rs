use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoadControllerRequest {
    pub name: ::std::string::String,
}

impl Default for LoadControllerRequest {
    fn default() -> Self {
        LoadControllerRequest {
            name: ::std::string::String::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for LoadControllerRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoadControllerResponse {
    pub ok: bool,
}

impl Default for LoadControllerResponse {
    fn default() -> Self {
        LoadControllerResponse {
            ok: false,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for LoadControllerResponse {}


pub struct LoadController;
