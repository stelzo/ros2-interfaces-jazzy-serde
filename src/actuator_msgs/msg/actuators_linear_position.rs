use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActuatorsLinearPosition {
    pub position: Vec<f64>,
}

impl Default for ActuatorsLinearPosition {
    fn default() -> Self {
        ActuatorsLinearPosition {
            position: Vec::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for ActuatorsLinearPosition {}
