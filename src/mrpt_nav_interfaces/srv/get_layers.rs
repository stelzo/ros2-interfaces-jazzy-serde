use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetLayersRequest {

}

impl Default for GetLayersRequest {
    fn default() -> Self {
        GetLayersRequest {

        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for GetLayersRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetLayersResponse {
    pub layers: Vec<::std::string::String>,
}

impl Default for GetLayersResponse {
    fn default() -> Self {
        GetLayersResponse {
            layers: Vec::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for GetLayersResponse {}


pub struct GetLayers;
