use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CONodeRequest {
    pub nodeid: u8,
}

impl Default for CONodeRequest {
    fn default() -> Self {
        CONodeRequest {
            nodeid: 0,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for CONodeRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CONodeResponse {
    pub success: bool,
}

impl Default for CONodeResponse {
    fn default() -> Self {
        CONodeResponse {
            success: false,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for CONodeResponse {}


pub struct CONode;
