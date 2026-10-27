use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct COHeartbeatIDRequest {
    pub nodeid: u8,
    pub heartbeat: u16,
}

impl Default for COHeartbeatIDRequest {
    fn default() -> Self {
        COHeartbeatIDRequest {
            nodeid: 0,
            heartbeat: 0,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for COHeartbeatIDRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct COHeartbeatIDResponse {
    pub success: bool,
}

impl Default for COHeartbeatIDResponse {
    fn default() -> Self {
        COHeartbeatIDResponse {
            success: false,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for COHeartbeatIDResponse {}


pub struct COHeartbeatID;
