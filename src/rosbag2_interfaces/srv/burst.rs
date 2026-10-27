use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BurstRequest {
    pub num_messages: u64,
}

impl Default for BurstRequest {
    fn default() -> Self {
        BurstRequest {
            num_messages: 0,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for BurstRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BurstResponse {
    pub actually_burst: u64,
}

impl Default for BurstResponse {
    fn default() -> Self {
        BurstResponse {
            actually_burst: 0,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for BurstResponse {}


pub struct Burst;
