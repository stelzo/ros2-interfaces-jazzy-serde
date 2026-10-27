use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteModelRequest {
    pub model_name: ::std::string::String,
}

impl Default for DeleteModelRequest {
    fn default() -> Self {
        DeleteModelRequest {
            model_name: ::std::string::String::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for DeleteModelRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteModelResponse {
    pub success: bool,
    pub status_message: ::std::string::String,
}

impl Default for DeleteModelResponse {
    fn default() -> Self {
        DeleteModelResponse {
            success: false,
            status_message: ::std::string::String::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for DeleteModelResponse {}


pub struct DeleteModel;
