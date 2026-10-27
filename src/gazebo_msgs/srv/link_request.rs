use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LinkRequestRequest {
    pub link_name: ::std::string::String,
}

impl Default for LinkRequestRequest {
    fn default() -> Self {
        LinkRequestRequest {
            link_name: ::std::string::String::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for LinkRequestRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LinkRequestResponse {

}

impl Default for LinkRequestResponse {
    fn default() -> Self {
        LinkRequestResponse {

        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for LinkRequestResponse {}


pub struct LinkRequest;
