use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestHeaderTwo {
    pub header: crate::std_msgs::msg::Header,
}

impl Default for TestHeaderTwo {
    fn default() -> Self {
        TestHeaderTwo {
            header: crate::std_msgs::msg::Header::default(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for TestHeaderTwo {}
