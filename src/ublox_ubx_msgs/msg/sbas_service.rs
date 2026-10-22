use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SBASService {
    pub ranging: bool,
    pub corrections: bool,
    pub integrity: bool,
    pub test_mode: bool,
    pub bad: bool,
}

impl Default for SBASService {
    fn default() -> Self {
        SBASService {
            ranging: false,
            corrections: false,
            integrity: false,
            test_mode: false,
            bad: false,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for SBASService {}
