use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetLabelRequest {
    pub node_id: i32,
    pub node_label: ::std::string::String,
}

impl Default for SetLabelRequest {
    fn default() -> Self {
        SetLabelRequest {
            node_id: 0,
            node_label: ::std::string::String::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for SetLabelRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetLabelResponse {

}

impl Default for SetLabelResponse {
    fn default() -> Self {
        SetLabelResponse {

        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for SetLabelResponse {}


pub struct SetLabel;
