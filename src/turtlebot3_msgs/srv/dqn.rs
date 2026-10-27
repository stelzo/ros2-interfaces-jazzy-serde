use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DqnRequest {
    pub action: u8,
    pub init: bool,
}

impl Default for DqnRequest {
    fn default() -> Self {
        DqnRequest {
            action: 0,
            init: false,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for DqnRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DqnResponse {
    pub state: Vec<f32>,
    pub reward: f32,
    pub done: bool,
}

impl Default for DqnResponse {
    fn default() -> Self {
        DqnResponse {
            state: Vec::new(),
            reward: 0.0,
            done: false,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for DqnResponse {}


pub struct Dqn;
