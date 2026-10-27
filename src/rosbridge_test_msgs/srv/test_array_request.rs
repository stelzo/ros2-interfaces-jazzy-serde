use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestArrayRequestRequest {
    pub int_values: Vec<i32>,
}

impl Default for TestArrayRequestRequest {
    fn default() -> Self {
        TestArrayRequestRequest {
            int_values: Vec::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for TestArrayRequestRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestArrayRequestResponse {

}

impl Default for TestArrayRequestResponse {
    fn default() -> Self {
        TestArrayRequestResponse {

        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for TestArrayRequestResponse {}


pub struct TestArrayRequest;
