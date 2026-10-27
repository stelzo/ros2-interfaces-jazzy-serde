use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetAvailableModesRequest {

}

impl Default for GetAvailableModesRequest {
    fn default() -> Self {
        GetAvailableModesRequest {

        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for GetAvailableModesRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetAvailableModesResponse {
    pub available_modes: Vec<::std::string::String>,
}

impl Default for GetAvailableModesResponse {
    fn default() -> Self {
        GetAvailableModesResponse {
            available_modes: Vec::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for GetAvailableModesResponse {}


pub struct GetAvailableModes;
