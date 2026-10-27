use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestResponseOnlyRequest {

}

impl Default for TestResponseOnlyRequest {
    fn default() -> Self {
        TestResponseOnlyRequest {

        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for TestResponseOnlyRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestResponseOnlyResponse {
    pub data: i32,
}

impl Default for TestResponseOnlyResponse {
    fn default() -> Self {
        TestResponseOnlyResponse {
            data: 0,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for TestResponseOnlyResponse {}


pub struct TestResponseOnly;
