use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NegotiationStates {
    pub negotiations: Vec<crate::rmf_traffic_msgs::msg::NegotiationState>,
}

impl Default for NegotiationStates {
    fn default() -> Self {
        NegotiationStates {
            negotiations: Vec::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for NegotiationStates {}
