use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IsConnectedRequest {

}

impl Default for IsConnectedRequest {
    fn default() -> Self {
        IsConnectedRequest {

        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for IsConnectedRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IsConnectedResponse {
    pub connected: bool,
}

impl Default for IsConnectedResponse {
    fn default() -> Self {
        IsConnectedResponse {
            connected: false,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for IsConnectedResponse {}


pub struct IsConnected;
