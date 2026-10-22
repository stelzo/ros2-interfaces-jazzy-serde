use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestHeaderArray {
    pub header: Vec<crate::std_msgs::msg::Header>,
}

impl Default for TestHeaderArray {
    fn default() -> Self {
        TestHeaderArray {
            header: Vec::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for TestHeaderArray {}
