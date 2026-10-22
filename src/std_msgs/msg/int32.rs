use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Int32 {
    pub data: i32,
}

impl Default for Int32 {
    fn default() -> Self {
        Int32 {
            data: 0,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for Int32 {}
