use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetDomainTypesRequest {
    pub request: crate::std_msgs::msg::Empty,
}

impl Default for GetDomainTypesRequest {
    fn default() -> Self {
        GetDomainTypesRequest {
            request: crate::std_msgs::msg::Empty::default(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for GetDomainTypesRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetDomainTypesResponse {
    pub success: bool,
    pub types: Vec<::std::string::String>,
    pub error_info: ::std::string::String,
}

impl Default for GetDomainTypesResponse {
    fn default() -> Self {
        GetDomainTypesResponse {
            success: false,
            types: Vec::new(),
            error_info: ::std::string::String::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for GetDomainTypesResponse {}


pub struct GetDomainTypes;
