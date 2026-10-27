use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BodyRequestRequest {
    pub body_name: ::std::string::String,
}

impl Default for BodyRequestRequest {
    fn default() -> Self {
        BodyRequestRequest {
            body_name: ::std::string::String::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for BodyRequestRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BodyRequestResponse {

}

impl Default for BodyRequestResponse {
    fn default() -> Self {
        BodyRequestResponse {

        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for BodyRequestResponse {}


pub struct BodyRequest;
