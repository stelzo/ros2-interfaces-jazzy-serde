use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteParamRequest {
    pub name: ::std::string::String,
}

impl Default for DeleteParamRequest {
    fn default() -> Self {
        DeleteParamRequest {
            name: ::std::string::String::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for DeleteParamRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteParamResponse {

}

impl Default for DeleteParamResponse {
    fn default() -> Self {
        DeleteParamResponse {

        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for DeleteParamResponse {}


pub struct DeleteParam;
