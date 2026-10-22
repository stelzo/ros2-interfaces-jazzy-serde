use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JointCommand {
    pub position: f64,
}

impl Default for JointCommand {
    fn default() -> Self {
        JointCommand {
            position: 0.0,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for JointCommand {}
