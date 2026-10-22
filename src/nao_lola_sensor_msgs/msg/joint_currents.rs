use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JointCurrents {
    pub currents: [f32; 25],
}

impl Default for JointCurrents {
    fn default() -> Self {
        JointCurrents {
            currents: [0.0; 25],
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for JointCurrents {}
