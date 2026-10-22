use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NegotiationStatuses {
    pub negotiations: Vec<crate::rmf_traffic_msgs::msg::NegotiationStatus>,
}

impl Default for NegotiationStatuses {
    fn default() -> Self {
        NegotiationStatuses {
            negotiations: Vec::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for NegotiationStatuses {}
