use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Int64 {
    pub data: i64,
}

impl Default for Int64 {
    fn default() -> Self {
        Int64 {
            data: 0,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for Int64 {}
