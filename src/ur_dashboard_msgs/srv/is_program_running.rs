use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IsProgramRunningRequest {

}

impl Default for IsProgramRunningRequest {
    fn default() -> Self {
        IsProgramRunningRequest {

        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for IsProgramRunningRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IsProgramRunningResponse {
    pub answer: ::std::string::String,
    pub program_running: bool,
    pub success: bool,
}

impl Default for IsProgramRunningResponse {
    fn default() -> Self {
        IsProgramRunningResponse {
            answer: ::std::string::String::new(),
            program_running: false,
            success: false,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for IsProgramRunningResponse {}


pub struct IsProgramRunning;
