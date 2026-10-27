use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestRequestAndResponseRequest {
    pub data: i32,
}

impl Default for TestRequestAndResponseRequest {
    fn default() -> Self {
        TestRequestAndResponseRequest {
            data: 0,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for TestRequestAndResponseRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestRequestAndResponseResponse {
    pub data: i32,
}

impl Default for TestRequestAndResponseResponse {
    fn default() -> Self {
        TestRequestAndResponseResponse {
            data: 0,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for TestRequestAndResponseResponse {}


pub struct TestRequestAndResponse;
