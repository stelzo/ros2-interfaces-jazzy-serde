use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorldReset {
    pub all: bool, // default: false
    pub time_only: bool, // default: false
    pub model_only: bool, // default: false
}

impl Default for WorldReset {
    fn default() -> Self {
        WorldReset {
            all: false,
            time_only: false,
            model_only: false,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for WorldReset {}
