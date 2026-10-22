use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TopicsForTypeRequest {
    #[serde(rename = "type")]    pub type_: ::std::string::String,
}

impl Default for TopicsForTypeRequest {
    fn default() -> Self {
        TopicsForTypeRequest {
            type_: ::std::string::String::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for TopicsForTypeRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TopicsForTypeResponse {
    pub topics: Vec<::std::string::String>,
}

impl Default for TopicsForTypeResponse {
    fn default() -> Self {
        TopicsForTypeResponse {
            topics: Vec::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for TopicsForTypeResponse {}


pub struct TopicsForType;
#[cfg(feature = "ros2-client")]
impl ros2_client::Service for TopicsForType {
    type Request = TopicsForTypeRequest;
    type Response = TopicsForTypeResponse;

    fn request_type_name(&self) -> &str { "TopicsForTypeRequest" }
    fn response_type_name(&self) -> &str { "TopicsForTypeResponse" }
}
