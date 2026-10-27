use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetParamRequest {
    pub name: ::std::string::String,
    pub value: ::std::string::String,
}

impl Default for SetParamRequest {
    fn default() -> Self {
        SetParamRequest {
            name: ::std::string::String::new(),
            value: ::std::string::String::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for SetParamRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetParamResponse {

}

impl Default for SetParamResponse {
    fn default() -> Self {
        SetParamResponse {

        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for SetParamResponse {}


pub struct SetParam;
