use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Char {
    pub data: i8,
}

impl Default for Char {
    fn default() -> Self {
        Char {
            data: 0,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for Char {}
