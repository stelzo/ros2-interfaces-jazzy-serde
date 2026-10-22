use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestTimeArray {
    pub times: Vec<crate::builtin_interfaces::msg::Time>,
}

impl Default for TestTimeArray {
    fn default() -> Self {
        TestTimeArray {
            times: Vec::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for TestTimeArray {}
