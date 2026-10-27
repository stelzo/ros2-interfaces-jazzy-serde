use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetBlackboardVariablesRequest {

}

impl Default for GetBlackboardVariablesRequest {
    fn default() -> Self {
        GetBlackboardVariablesRequest {

        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for GetBlackboardVariablesRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetBlackboardVariablesResponse {
    pub variables: Vec<::std::string::String>,
}

impl Default for GetBlackboardVariablesResponse {
    fn default() -> Self {
        GetBlackboardVariablesResponse {
            variables: Vec::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for GetBlackboardVariablesResponse {}


pub struct GetBlackboardVariables;
