use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EndpointAddRequest {
    pub url: ::std::string::String,
    #[serde(rename = "type")]    pub type_: u8,
}

impl EndpointAddRequest {
    pub const TYPE_FCU: u8 = 0;
    pub const TYPE_GCS: u8 = 1;
    pub const TYPE_UAS: u8 = 2;
}

impl Default for EndpointAddRequest {
    fn default() -> Self {
        EndpointAddRequest {
            url: ::std::string::String::new(),
            type_: 0,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for EndpointAddRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EndpointAddResponse {
    pub successful: bool,
    pub reason: ::std::string::String,
    pub id: u32,
}

impl Default for EndpointAddResponse {
    fn default() -> Self {
        EndpointAddResponse {
            successful: false,
            reason: ::std::string::String::new(),
            id: 0,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for EndpointAddResponse {}


pub struct EndpointAdd;
