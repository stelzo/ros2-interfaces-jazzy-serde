use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResetODORequest {

}

impl Default for ResetODORequest {
    fn default() -> Self {
        ResetODORequest {

        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for ResetODORequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResetODOResponse {

}

impl Default for ResetODOResponse {
    fn default() -> Self {
        ResetODOResponse {

        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for ResetODOResponse {}


pub struct ResetODO;
