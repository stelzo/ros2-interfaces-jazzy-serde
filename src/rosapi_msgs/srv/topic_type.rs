use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TopicTypeRequest {
    pub topic: ::std::string::String,
}

impl Default for TopicTypeRequest {
    fn default() -> Self {
        TopicTypeRequest {
            topic: ::std::string::String::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for TopicTypeRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TopicTypeResponse {
    #[serde(rename = "type")]    pub type_: ::std::string::String,
}

impl Default for TopicTypeResponse {
    fn default() -> Self {
        TopicTypeResponse {
            type_: ::std::string::String::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for TopicTypeResponse {}


pub struct TopicType;
