use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListLabelsRequest {

}

impl Default for ListLabelsRequest {
    fn default() -> Self {
        ListLabelsRequest {

        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for ListLabelsRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListLabelsResponse {
    pub ids: Vec<i32>,
    pub labels: Vec<::std::string::String>,
}

impl Default for ListLabelsResponse {
    fn default() -> Self {
        ListLabelsResponse {
            ids: Vec::new(),
            labels: Vec::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for ListLabelsResponse {}


pub struct ListLabels;
