use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteRouteRequest {
    pub guid: ::std::string::String,
}

impl Default for DeleteRouteRequest {
    fn default() -> Self {
        DeleteRouteRequest {
            guid: ::std::string::String::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for DeleteRouteRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteRouteResponse {
    pub success: bool,
    pub message: ::std::string::String,
}

impl Default for DeleteRouteResponse {
    fn default() -> Self {
        DeleteRouteResponse {
            success: false,
            message: ::std::string::String::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for DeleteRouteResponse {}


pub struct DeleteRoute;
