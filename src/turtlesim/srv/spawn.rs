use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpawnRequest {
    pub x: f32,
    pub y: f32,
    pub theta: f32,
    pub name: ::std::string::String,
}

impl Default for SpawnRequest {
    fn default() -> Self {
        SpawnRequest {
            x: 0.0,
            y: 0.0,
            theta: 0.0,
            name: ::std::string::String::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for SpawnRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpawnResponse {
    pub name: ::std::string::String,
}

impl Default for SpawnResponse {
    fn default() -> Self {
        SpawnResponse {
            name: ::std::string::String::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for SpawnResponse {}


pub struct Spawn;
