use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestHeader {
    pub header: crate::std_msgs::msg::Header,
}

impl Default for TestHeader {
    fn default() -> Self {
        TestHeader {
            header: crate::std_msgs::msg::Header::default(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for TestHeader {}
