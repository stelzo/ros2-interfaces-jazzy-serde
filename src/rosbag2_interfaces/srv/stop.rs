use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StopRequest {

}

impl Default for StopRequest {
    fn default() -> Self {
        StopRequest {

        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for StopRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StopResponse {

}

impl Default for StopResponse {
    fn default() -> Self {
        StopResponse {

        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for StopResponse {}


pub struct Stop;
