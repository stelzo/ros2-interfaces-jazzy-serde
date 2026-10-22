use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StopStatus {
    pub header: crate::std_msgs::msg::Header,
    pub is_stopped: bool,
}

impl Default for StopStatus {
    fn default() -> Self {
        StopStatus {
            header: crate::std_msgs::msg::Header::default(),
            is_stopped: false,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for StopStatus {}
