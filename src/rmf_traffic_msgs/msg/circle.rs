use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Circle {
    pub radius: f64,
}

impl Default for Circle {
    fn default() -> Self {
        Circle {
            radius: 0.0,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for Circle {}
