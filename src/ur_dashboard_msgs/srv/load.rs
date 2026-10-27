use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoadRequest {
    pub filename: ::std::string::String,
}

impl Default for LoadRequest {
    fn default() -> Self {
        LoadRequest {
            filename: ::std::string::String::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for LoadRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoadResponse {
    pub answer: ::std::string::String,
    pub success: bool,
}

impl Default for LoadResponse {
    fn default() -> Self {
        LoadResponse {
            answer: ::std::string::String::new(),
            success: false,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for LoadResponse {}


pub struct Load;
