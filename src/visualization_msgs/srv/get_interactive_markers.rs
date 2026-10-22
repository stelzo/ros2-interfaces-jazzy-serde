use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetInteractiveMarkersRequest {

}

impl Default for GetInteractiveMarkersRequest {
    fn default() -> Self {
        GetInteractiveMarkersRequest {

        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for GetInteractiveMarkersRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetInteractiveMarkersResponse {
    pub sequence_number: u64,
    pub markers: Vec<crate::visualization_msgs::msg::InteractiveMarker>,
}

impl Default for GetInteractiveMarkersResponse {
    fn default() -> Self {
        GetInteractiveMarkersResponse {
            sequence_number: 0,
            markers: Vec::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for GetInteractiveMarkersResponse {}


pub struct GetInteractiveMarkers;
#[cfg(feature = "ros2-client")]
impl ros2_client::Service for GetInteractiveMarkers {
    type Request = GetInteractiveMarkersRequest;
    type Response = GetInteractiveMarkersResponse;

    fn request_type_name(&self) -> &str { "GetInteractiveMarkersRequest" }
    fn response_type_name(&self) -> &str { "GetInteractiveMarkersResponse" }
}
