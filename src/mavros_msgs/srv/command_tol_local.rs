use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommandTOLLocalRequest {
    pub min_pitch: f32,
    pub offset: f32,
    pub rate: f32,
    pub yaw: f32,
    pub position: crate::geometry_msgs::msg::Vector3,
}

impl Default for CommandTOLLocalRequest {
    fn default() -> Self {
        CommandTOLLocalRequest {
            min_pitch: 0.0,
            offset: 0.0,
            rate: 0.0,
            yaw: 0.0,
            position: crate::geometry_msgs::msg::Vector3::default(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for CommandTOLLocalRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommandTOLLocalResponse {
    pub success: bool,
    pub result: u8,
}

impl Default for CommandTOLLocalResponse {
    fn default() -> Self {
        CommandTOLLocalResponse {
            success: false,
            result: 0,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for CommandTOLLocalResponse {}


pub struct CommandTOLLocal;
