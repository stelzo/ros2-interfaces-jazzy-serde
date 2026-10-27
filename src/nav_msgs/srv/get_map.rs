use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMapRequest {

}

impl Default for GetMapRequest {
    fn default() -> Self {
        GetMapRequest {

        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for GetMapRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMapResponse {
    pub map: crate::nav_msgs::msg::OccupancyGrid,
}

impl Default for GetMapResponse {
    fn default() -> Self {
        GetMapResponse {
            map: crate::nav_msgs::msg::OccupancyGrid::default(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for GetMapResponse {}


pub struct GetMap;
