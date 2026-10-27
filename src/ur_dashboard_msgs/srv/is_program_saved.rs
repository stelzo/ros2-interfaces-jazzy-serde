use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IsProgramSavedRequest {

}

impl Default for IsProgramSavedRequest {
    fn default() -> Self {
        IsProgramSavedRequest {

        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for IsProgramSavedRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IsProgramSavedResponse {
    pub answer: ::std::string::String,
    pub program_name: ::std::string::String,
    pub program_saved: bool,
    pub success: bool,
}

impl Default for IsProgramSavedResponse {
    fn default() -> Self {
        IsProgramSavedResponse {
            answer: ::std::string::String::new(),
            program_name: ::std::string::String::new(),
            program_saved: false,
            success: false,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for IsProgramSavedResponse {}


pub struct IsProgramSaved;
