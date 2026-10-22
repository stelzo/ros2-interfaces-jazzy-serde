use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComplexMsgDependsOnIdl {
    pub a: crate::rosbag2_test_msgdefs::msg::BasicIdl,
}

impl Default for ComplexMsgDependsOnIdl {
    fn default() -> Self {
        ComplexMsgDependsOnIdl {
            a: crate::rosbag2_test_msgdefs::msg::BasicIdl::default(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for ComplexMsgDependsOnIdl {}
