use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetRouteListRequest {

}

impl Default for GetRouteListRequest {
    fn default() -> Self {
        GetRouteListRequest {

        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for GetRouteListRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetRouteListResponse {
    pub routes: Vec<crate::marti_nav_msgs::msg::Route>,
    pub success: bool,
    pub message: ::std::string::String,
}

impl Default for GetRouteListResponse {
    fn default() -> Self {
        GetRouteListResponse {
            routes: Vec::new(),
            success: false,
            message: ::std::string::String::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for GetRouteListResponse {}


pub struct GetRouteList;
