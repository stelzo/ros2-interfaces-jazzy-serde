use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpeedLimitedLane {
    pub lane_index: u64,
    pub speed_limit: f64,
}

impl Default for SpeedLimitedLane {
    fn default() -> Self {
        SpeedLimitedLane {
            lane_index: 0,
            speed_limit: 0.0,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for SpeedLimitedLane {}
