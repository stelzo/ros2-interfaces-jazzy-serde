use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClearProblemKnowledgeRequest {
    pub request: crate::std_msgs::msg::Empty,
}

impl Default for ClearProblemKnowledgeRequest {
    fn default() -> Self {
        ClearProblemKnowledgeRequest {
            request: crate::std_msgs::msg::Empty::default(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for ClearProblemKnowledgeRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClearProblemKnowledgeResponse {
    pub success: bool,
    pub error_info: ::std::string::String,
}

impl Default for ClearProblemKnowledgeResponse {
    fn default() -> Self {
        ClearProblemKnowledgeResponse {
            success: false,
            error_info: ::std::string::String::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for ClearProblemKnowledgeResponse {}


pub struct ClearProblemKnowledge;
