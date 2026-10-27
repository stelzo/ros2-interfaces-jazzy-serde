use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectedMapsInfoRequest {
    pub projected_maps_info: Vec<crate::map_msgs::msg::ProjectedMapInfo>,
}

impl Default for ProjectedMapsInfoRequest {
    fn default() -> Self {
        ProjectedMapsInfoRequest {
            projected_maps_info: Vec::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for ProjectedMapsInfoRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectedMapsInfoResponse {

}

impl Default for ProjectedMapsInfoResponse {
    fn default() -> Self {
        ProjectedMapsInfoResponse {

        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for ProjectedMapsInfoResponse {}


pub struct ProjectedMapsInfo;
