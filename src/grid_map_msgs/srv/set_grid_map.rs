use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetGridMapRequest {
    pub map: crate::grid_map_msgs::msg::GridMap,
}

impl Default for SetGridMapRequest {
    fn default() -> Self {
        SetGridMapRequest {
            map: crate::grid_map_msgs::msg::GridMap::default(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for SetGridMapRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetGridMapResponse {

}

impl Default for SetGridMapResponse {
    fn default() -> Self {
        SetGridMapResponse {

        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for SetGridMapResponse {}


pub struct SetGridMap;
