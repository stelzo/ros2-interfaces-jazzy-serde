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
