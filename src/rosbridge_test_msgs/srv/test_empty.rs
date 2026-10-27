use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestEmptyRequest {

}

impl Default for TestEmptyRequest {
    fn default() -> Self {
        TestEmptyRequest {

        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for TestEmptyRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestEmptyResponse {

}

impl Default for TestEmptyResponse {
    fn default() -> Self {
        TestEmptyResponse {

        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for TestEmptyResponse {}


pub struct TestEmpty;
