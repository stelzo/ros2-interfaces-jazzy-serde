use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetNodeDetailsRequest {
    pub expression: ::std::string::String,
}

impl Default for GetNodeDetailsRequest {
    fn default() -> Self {
        GetNodeDetailsRequest {
            expression: ::std::string::String::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for GetNodeDetailsRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetNodeDetailsResponse {
    pub success: bool,
    pub node: crate::plansys2_msgs::msg::Node,
    pub error_info: ::std::string::String,
}

impl Default for GetNodeDetailsResponse {
    fn default() -> Self {
        GetNodeDetailsResponse {
            success: false,
            node: crate::plansys2_msgs::msg::Node::default(),
            error_info: ::std::string::String::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for GetNodeDetailsResponse {}


pub struct GetNodeDetails;
