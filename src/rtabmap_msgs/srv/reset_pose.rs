use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResetPoseRequest {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub roll: f32,
    pub pitch: f32,
    pub yaw: f32,
}

impl Default for ResetPoseRequest {
    fn default() -> Self {
        ResetPoseRequest {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            roll: 0.0,
            pitch: 0.0,
            yaw: 0.0,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for ResetPoseRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResetPoseResponse {

}

impl Default for ResetPoseResponse {
    fn default() -> Self {
        ResetPoseResponse {

        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for ResetPoseResponse {}


pub struct ResetPose;
