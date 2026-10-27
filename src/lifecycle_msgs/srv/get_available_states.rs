use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetAvailableStatesRequest {

}

impl Default for GetAvailableStatesRequest {
    fn default() -> Self {
        GetAvailableStatesRequest {

        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for GetAvailableStatesRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetAvailableStatesResponse {
    pub available_states: Vec<crate::lifecycle_msgs::msg::State>,
}

impl Default for GetAvailableStatesResponse {
    fn default() -> Self {
        GetAvailableStatesResponse {
            available_states: Vec::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for GetAvailableStatesResponse {}


pub struct GetAvailableStates;
