use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AffectNodeRequest {
    pub node: crate::plansys2_msgs::msg::Node,
}

impl Default for AffectNodeRequest {
    fn default() -> Self {
        AffectNodeRequest {
            node: crate::plansys2_msgs::msg::Node::default(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for AffectNodeRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AffectNodeResponse {
    pub success: bool,
    pub error_info: ::std::string::String,
}

impl Default for AffectNodeResponse {
    fn default() -> Self {
        AffectNodeResponse {
            success: false,
            error_info: ::std::string::String::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for AffectNodeResponse {}


pub struct AffectNode;
