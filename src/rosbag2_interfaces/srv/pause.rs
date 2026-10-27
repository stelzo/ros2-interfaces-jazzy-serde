use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PauseRequest {

}

impl Default for PauseRequest {
    fn default() -> Self {
        PauseRequest {

        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for PauseRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PauseResponse {

}

impl Default for PauseResponse {
    fn default() -> Self {
        PauseResponse {

        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for PauseResponse {}


pub struct Pause;
