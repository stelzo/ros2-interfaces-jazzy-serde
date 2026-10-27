use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IsPausedRequest {

}

impl Default for IsPausedRequest {
    fn default() -> Self {
        IsPausedRequest {

        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for IsPausedRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IsPausedResponse {
    pub paused: bool,
}

impl Default for IsPausedResponse {
    fn default() -> Self {
        IsPausedResponse {
            paused: false,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for IsPausedResponse {}


pub struct IsPaused;
