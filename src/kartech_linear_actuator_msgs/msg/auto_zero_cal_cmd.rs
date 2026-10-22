use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AutoZeroCalCmd {
    pub header: crate::std_msgs::msg::Header,
    pub confirm: bool,
    pub auto_reply: bool,
}

impl Default for AutoZeroCalCmd {
    fn default() -> Self {
        AutoZeroCalCmd {
            header: crate::std_msgs::msg::Header::default(),
            confirm: false,
            auto_reply: false,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for AutoZeroCalCmd {}
