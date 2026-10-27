use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SplitBagfileRequest {

}

impl Default for SplitBagfileRequest {
    fn default() -> Self {
        SplitBagfileRequest {

        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for SplitBagfileRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SplitBagfileResponse {

}

impl Default for SplitBagfileResponse {
    fn default() -> Self {
        SplitBagfileResponse {

        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for SplitBagfileResponse {}


pub struct SplitBagfile;
