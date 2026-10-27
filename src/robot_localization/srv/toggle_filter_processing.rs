use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToggleFilterProcessingRequest {
    pub on: bool,
}

impl Default for ToggleFilterProcessingRequest {
    fn default() -> Self {
        ToggleFilterProcessingRequest {
            on: false,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for ToggleFilterProcessingRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToggleFilterProcessingResponse {
    pub status: bool,
}

impl Default for ToggleFilterProcessingResponse {
    fn default() -> Self {
        ToggleFilterProcessingResponse {
            status: false,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for ToggleFilterProcessingResponse {}


pub struct ToggleFilterProcessing;
