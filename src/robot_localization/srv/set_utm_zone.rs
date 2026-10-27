use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetUTMZoneRequest {
    pub utm_zone: ::std::string::String,
}

impl Default for SetUTMZoneRequest {
    fn default() -> Self {
        SetUTMZoneRequest {
            utm_zone: ::std::string::String::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for SetUTMZoneRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetUTMZoneResponse {

}

impl Default for SetUTMZoneResponse {
    fn default() -> Self {
        SetUTMZoneResponse {

        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for SetUTMZoneResponse {}


pub struct SetUTMZone;
