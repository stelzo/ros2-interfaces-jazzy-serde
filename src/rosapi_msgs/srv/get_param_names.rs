use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetParamNamesRequest {

}

impl Default for GetParamNamesRequest {
    fn default() -> Self {
        GetParamNamesRequest {

        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for GetParamNamesRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetParamNamesResponse {
    pub names: Vec<::std::string::String>,
}

impl Default for GetParamNamesResponse {
    fn default() -> Self {
        GetParamNamesResponse {
            names: Vec::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for GetParamNamesResponse {}


pub struct GetParamNames;
