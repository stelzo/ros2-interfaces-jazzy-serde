use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HandEyeCalibrationTriggerRequest {

}

impl Default for HandEyeCalibrationTriggerRequest {
    fn default() -> Self {
        HandEyeCalibrationTriggerRequest {

        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for HandEyeCalibrationTriggerRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HandEyeCalibrationTriggerResponse {
    pub success: bool,
    pub status: i32,
    pub message: ::std::string::String,
}

impl Default for HandEyeCalibrationTriggerResponse {
    fn default() -> Self {
        HandEyeCalibrationTriggerResponse {
            success: false,
            status: 0,
            message: ::std::string::String::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for HandEyeCalibrationTriggerResponse {}


pub struct HandEyeCalibrationTrigger;
