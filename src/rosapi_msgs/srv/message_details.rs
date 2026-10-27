use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessageDetailsRequest {
    #[serde(rename = "type")]    pub type_: ::std::string::String,
}

impl Default for MessageDetailsRequest {
    fn default() -> Self {
        MessageDetailsRequest {
            type_: ::std::string::String::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for MessageDetailsRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessageDetailsResponse {
    pub typedefs: Vec<crate::rosapi_msgs::msg::TypeDef>,
}

impl Default for MessageDetailsResponse {
    fn default() -> Self {
        MessageDetailsResponse {
            typedefs: Vec::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for MessageDetailsResponse {}


pub struct MessageDetails;
