use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SmaccGetTransitionHistoryRequest {

}

impl Default for SmaccGetTransitionHistoryRequest {
    fn default() -> Self {
        SmaccGetTransitionHistoryRequest {

        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for SmaccGetTransitionHistoryRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SmaccGetTransitionHistoryResponse {
    pub history: Vec<crate::smacc2_msgs::msg::SmaccTransitionLogEntry>,
}

impl Default for SmaccGetTransitionHistoryResponse {
    fn default() -> Self {
        SmaccGetTransitionHistoryResponse {
            history: Vec::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for SmaccGetTransitionHistoryResponse {}


pub struct SmaccGetTransitionHistory;
