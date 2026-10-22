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
#[cfg(feature = "ros2-client")]
impl ros2_client::Service for TestEmpty {
    type Request = TestEmptyRequest;
    type Response = TestEmptyResponse;

    fn request_type_name(&self) -> &str { "TestEmptyRequest" }
    fn response_type_name(&self) -> &str { "TestEmptyResponse" }
}
