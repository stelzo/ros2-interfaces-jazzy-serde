use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TeleportRelativeRequest {
    pub linear: f32,
    pub angular: f32,
}

impl Default for TeleportRelativeRequest {
    fn default() -> Self {
        TeleportRelativeRequest {
            linear: 0.0,
            angular: 0.0,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for TeleportRelativeRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TeleportRelativeResponse {

}

impl Default for TeleportRelativeResponse {
    fn default() -> Self {
        TeleportRelativeResponse {

        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for TeleportRelativeResponse {}


pub struct TeleportRelative;
