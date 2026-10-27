use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddProblemRequest {
    pub problem: ::std::string::String,
}

impl Default for AddProblemRequest {
    fn default() -> Self {
        AddProblemRequest {
            problem: ::std::string::String::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for AddProblemRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddProblemResponse {
    pub success: bool,
    pub error_info: ::std::string::String,
}

impl Default for AddProblemResponse {
    fn default() -> Self {
        AddProblemResponse {
            success: false,
            error_info: ::std::string::String::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for AddProblemResponse {}


pub struct AddProblem;
