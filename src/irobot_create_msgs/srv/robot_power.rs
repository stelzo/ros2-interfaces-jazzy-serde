use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RobotPowerRequest {

}

impl Default for RobotPowerRequest {
    fn default() -> Self {
        RobotPowerRequest {

        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for RobotPowerRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RobotPowerResponse {
    pub success: bool,
    pub message: ::std::string::String,
}

impl Default for RobotPowerResponse {
    fn default() -> Self {
        RobotPowerResponse {
            success: false,
            message: ::std::string::String::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for RobotPowerResponse {}


pub struct RobotPower;
