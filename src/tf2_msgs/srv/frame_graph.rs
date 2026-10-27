use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FrameGraphRequest {

}

impl Default for FrameGraphRequest {
    fn default() -> Self {
        FrameGraphRequest {

        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for FrameGraphRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FrameGraphResponse {
    pub frame_yaml: ::std::string::String,
}

impl Default for FrameGraphResponse {
    fn default() -> Self {
        FrameGraphResponse {
            frame_yaml: ::std::string::String::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for FrameGraphResponse {}


pub struct FrameGraph;
