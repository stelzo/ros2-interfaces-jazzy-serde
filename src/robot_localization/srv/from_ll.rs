use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FromLLRequest {
    pub ll_point: crate::geographic_msgs::msg::GeoPoint,
}

impl Default for FromLLRequest {
    fn default() -> Self {
        FromLLRequest {
            ll_point: crate::geographic_msgs::msg::GeoPoint::default(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for FromLLRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FromLLResponse {
    pub map_point: crate::geometry_msgs::msg::Point,
}

impl Default for FromLLResponse {
    fn default() -> Self {
        FromLLResponse {
            map_point: crate::geometry_msgs::msg::Point::default(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for FromLLResponse {}


pub struct FromLL;
