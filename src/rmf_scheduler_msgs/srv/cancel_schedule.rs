use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CancelScheduleRequest {
    pub name: ::std::string::String,
    pub finished: bool,
}

impl Default for CancelScheduleRequest {
    fn default() -> Self {
        CancelScheduleRequest {
            name: ::std::string::String::new(),
            finished: false,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for CancelScheduleRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CancelScheduleResponse {
    pub success: bool,
    pub message: ::std::string::String,
}

impl Default for CancelScheduleResponse {
    fn default() -> Self {
        CancelScheduleResponse {
            success: false,
            message: ::std::string::String::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for CancelScheduleResponse {}


pub struct CancelSchedule;
