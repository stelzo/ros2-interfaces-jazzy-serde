use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CompressedMsg {
    pub data: Vec<u8>,
}

impl Default for CompressedMsg {
    fn default() -> Self {
        CompressedMsg {
            data: Vec::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for CompressedMsg {}
