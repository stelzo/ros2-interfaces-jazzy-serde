use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayNextRequest {

}

impl Default for PlayNextRequest {
    fn default() -> Self {
        PlayNextRequest {

        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for PlayNextRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayNextResponse {
    pub success: bool,
}

impl Default for PlayNextResponse {
    fn default() -> Self {
        PlayNextResponse {
            success: false,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for PlayNextResponse {}


pub struct PlayNext;
