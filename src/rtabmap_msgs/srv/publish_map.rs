use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PublishMapRequest {
    pub global_map: bool,
    pub optimized: bool,
    pub graph_only: bool,
}

impl Default for PublishMapRequest {
    fn default() -> Self {
        PublishMapRequest {
            global_map: false,
            optimized: false,
            graph_only: false,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for PublishMapRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PublishMapResponse {

}

impl Default for PublishMapResponse {
    fn default() -> Self {
        PublishMapResponse {

        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for PublishMapResponse {}


pub struct PublishMap;
