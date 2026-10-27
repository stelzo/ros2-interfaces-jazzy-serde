use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetGeographicMapRequest {
    pub url: ::std::string::String,
    pub bounds: crate::geographic_msgs::msg::BoundingBox,
}

impl Default for GetGeographicMapRequest {
    fn default() -> Self {
        GetGeographicMapRequest {
            url: ::std::string::String::new(),
            bounds: crate::geographic_msgs::msg::BoundingBox::default(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for GetGeographicMapRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetGeographicMapResponse {
    pub success: bool,
    pub status: ::std::string::String,
    pub map: crate::geographic_msgs::msg::GeographicMap,
}

impl Default for GetGeographicMapResponse {
    fn default() -> Self {
        GetGeographicMapResponse {
            success: false,
            status: ::std::string::String::new(),
            map: crate::geographic_msgs::msg::GeographicMap::default(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for GetGeographicMapResponse {}


pub struct GetGeographicMap;
