use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestMultipleRequestFieldsRequest {
    pub int_value: i32,
    pub float_value: f32,
    pub string: ::std::string::String,
    pub bool_value: bool,
}

impl Default for TestMultipleRequestFieldsRequest {
    fn default() -> Self {
        TestMultipleRequestFieldsRequest {
            int_value: 0,
            float_value: 0.0,
            string: ::std::string::String::new(),
            bool_value: false,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for TestMultipleRequestFieldsRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestMultipleRequestFieldsResponse {

}

impl Default for TestMultipleRequestFieldsResponse {
    fn default() -> Self {
        TestMultipleRequestFieldsResponse {

        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for TestMultipleRequestFieldsResponse {}


pub struct TestMultipleRequestFields;
