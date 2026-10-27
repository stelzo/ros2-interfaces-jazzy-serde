use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetModeRequest {

}

impl Default for GetModeRequest {
    fn default() -> Self {
        GetModeRequest {

        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for GetModeRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetModeResponse {
    pub current_mode: ::std::string::String,
}

impl Default for GetModeResponse {
    fn default() -> Self {
        GetModeResponse {
            current_mode: ::std::string::String::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for GetModeResponse {}


pub struct GetMode;
