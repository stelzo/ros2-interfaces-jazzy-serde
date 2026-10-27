use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetDispatchStatesRequest {
    pub task_ids: Vec<::std::string::String>,
}

impl Default for GetDispatchStatesRequest {
    fn default() -> Self {
        GetDispatchStatesRequest {
            task_ids: Vec::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for GetDispatchStatesRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetDispatchStatesResponse {
    pub success: bool,
    pub states: crate::rmf_task_msgs::msg::DispatchStates,
}

impl Default for GetDispatchStatesResponse {
    fn default() -> Self {
        GetDispatchStatesResponse {
            success: false,
            states: crate::rmf_task_msgs::msg::DispatchStates::default(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for GetDispatchStatesResponse {}


pub struct GetDispatchStates;
