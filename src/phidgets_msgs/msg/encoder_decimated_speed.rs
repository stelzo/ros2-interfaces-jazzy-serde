use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EncoderDecimatedSpeed {
    pub header: crate::std_msgs::msg::Header,
    pub avr_speed: f64,
}

impl Default for EncoderDecimatedSpeed {
    fn default() -> Self {
        EncoderDecimatedSpeed {
            header: crate::std_msgs::msg::Header::default(),
            avr_speed: 0.0,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for EncoderDecimatedSpeed {}
