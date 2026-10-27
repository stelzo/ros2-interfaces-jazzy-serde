use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LIDoutputstateSrvRequest {
    pub active: bool,
}

impl Default for LIDoutputstateSrvRequest {
    fn default() -> Self {
        LIDoutputstateSrvRequest {
            active: false,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for LIDoutputstateSrvRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LIDoutputstateSrvResponse {
    pub success: bool,
}

impl Default for LIDoutputstateSrvResponse {
    fn default() -> Self {
        LIDoutputstateSrvResponse {
            success: false,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for LIDoutputstateSrvResponse {}


pub struct LIDoutputstateSrv;
