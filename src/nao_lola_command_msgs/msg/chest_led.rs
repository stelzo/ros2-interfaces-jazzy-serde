use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChestLed {
    pub color: crate::std_msgs::msg::ColorRGBA,
}

impl Default for ChestLed {
    fn default() -> Self {
        ChestLed {
            color: crate::std_msgs::msg::ColorRGBA::default(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for ChestLed {}
