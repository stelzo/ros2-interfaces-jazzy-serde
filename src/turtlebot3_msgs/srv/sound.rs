use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SoundRequest {
    pub value: u8,
}

impl Default for SoundRequest {
    fn default() -> Self {
        SoundRequest {
            value: 0,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for SoundRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SoundResponse {
    pub success: bool,
    pub message: ::std::string::String,
}

impl Default for SoundResponse {
    fn default() -> Self {
        SoundResponse {
            success: false,
            message: ::std::string::String::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for SoundResponse {}


pub struct Sound;
