use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClearEntireCostmapRequest {
    pub request: crate::std_msgs::msg::Empty,
}

impl Default for ClearEntireCostmapRequest {
    fn default() -> Self {
        ClearEntireCostmapRequest {
            request: crate::std_msgs::msg::Empty::default(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for ClearEntireCostmapRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClearEntireCostmapResponse {
    pub response: crate::std_msgs::msg::Empty,
}

impl Default for ClearEntireCostmapResponse {
    fn default() -> Self {
        ClearEntireCostmapResponse {
            response: crate::std_msgs::msg::Empty::default(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for ClearEntireCostmapResponse {}


pub struct ClearEntireCostmap;
