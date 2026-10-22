use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestChar {
    pub data: Vec<i8>,
}

impl Default for TestChar {
    fn default() -> Self {
        TestChar {
            data: Vec::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for TestChar {}
