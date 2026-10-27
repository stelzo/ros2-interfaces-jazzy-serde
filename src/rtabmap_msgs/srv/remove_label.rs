use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RemoveLabelRequest {
    pub label: ::std::string::String,
}

impl Default for RemoveLabelRequest {
    fn default() -> Self {
        RemoveLabelRequest {
            label: ::std::string::String::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for RemoveLabelRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RemoveLabelResponse {

}

impl Default for RemoveLabelResponse {
    fn default() -> Self {
        RemoveLabelResponse {

        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for RemoveLabelResponse {}


pub struct RemoveLabel;
