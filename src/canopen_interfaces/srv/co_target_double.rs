use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct COTargetDoubleRequest {
    pub target: f64,
}

impl Default for COTargetDoubleRequest {
    fn default() -> Self {
        COTargetDoubleRequest {
            target: 0.0,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for COTargetDoubleRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct COTargetDoubleResponse {
    pub success: bool,
}

impl Default for COTargetDoubleResponse {
    fn default() -> Self {
        COTargetDoubleResponse {
            success: false,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for COTargetDoubleResponse {}


pub struct COTargetDouble;
