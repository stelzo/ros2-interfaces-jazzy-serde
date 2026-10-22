use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KillRequest {
    pub name: ::std::string::String,
}

impl Default for KillRequest {
    fn default() -> Self {
        KillRequest {
            name: ::std::string::String::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for KillRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KillResponse {

}

impl Default for KillResponse {
    fn default() -> Self {
        KillResponse {

        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for KillResponse {}


pub struct Kill;
#[cfg(feature = "ros2-client")]
impl ros2_client::Service for Kill {
    type Request = KillRequest;
    type Response = KillResponse;

    fn request_type_name(&self) -> &str { "KillRequest" }
    fn response_type_name(&self) -> &str { "KillResponse" }
}
