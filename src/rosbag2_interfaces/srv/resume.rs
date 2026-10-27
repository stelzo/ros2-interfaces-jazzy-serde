use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResumeRequest {

}

impl Default for ResumeRequest {
    fn default() -> Self {
        ResumeRequest {

        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for ResumeRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResumeResponse {

}

impl Default for ResumeResponse {
    fn default() -> Self {
        ResumeResponse {

        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for ResumeResponse {}


pub struct Resume;
