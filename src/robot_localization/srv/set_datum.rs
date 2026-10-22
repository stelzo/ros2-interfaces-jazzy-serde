use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetDatumRequest {
    pub geo_pose: crate::geographic_msgs::msg::GeoPose,
}

impl Default for SetDatumRequest {
    fn default() -> Self {
        SetDatumRequest {
            geo_pose: crate::geographic_msgs::msg::GeoPose::default(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for SetDatumRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetDatumResponse {

}

impl Default for SetDatumResponse {
    fn default() -> Self {
        SetDatumResponse {

        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for SetDatumResponse {}


pub struct SetDatum;
#[cfg(feature = "ros2-client")]
impl ros2_client::Service for SetDatum {
    type Request = SetDatumRequest;
    type Response = SetDatumResponse;

    fn request_type_name(&self) -> &str { "SetDatumRequest" }
    fn response_type_name(&self) -> &str { "SetDatumResponse" }
}
