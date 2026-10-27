use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SaveMapRequest {
    pub filename: crate::std_msgs::msg::String,
}

impl Default for SaveMapRequest {
    fn default() -> Self {
        SaveMapRequest {
            filename: crate::std_msgs::msg::String::default(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for SaveMapRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SaveMapResponse {

}

impl Default for SaveMapResponse {
    fn default() -> Self {
        SaveMapResponse {

        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for SaveMapResponse {}


pub struct SaveMap;
