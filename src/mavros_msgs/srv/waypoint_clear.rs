use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WaypointClearRequest {

}

impl Default for WaypointClearRequest {
    fn default() -> Self {
        WaypointClearRequest {

        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for WaypointClearRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WaypointClearResponse {
    pub success: bool,
}

impl Default for WaypointClearResponse {
    fn default() -> Self {
        WaypointClearResponse {
            success: false,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for WaypointClearResponse {}


pub struct WaypointClear;
