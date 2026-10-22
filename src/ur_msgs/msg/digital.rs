use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Digital {
    pub pin: u8,
    pub state: bool,
}

impl Default for Digital {
    fn default() -> Self {
        Digital {
            pin: 0,
            state: false,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for Digital {}
