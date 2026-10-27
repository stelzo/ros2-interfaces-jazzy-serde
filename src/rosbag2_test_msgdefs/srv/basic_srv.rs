use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BasicSrvRequest {
    pub req: ::std::string::String,
}

impl Default for BasicSrvRequest {
    fn default() -> Self {
        BasicSrvRequest {
            req: ::std::string::String::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for BasicSrvRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BasicSrvResponse {
    pub resp: ::std::string::String,
}

impl Default for BasicSrvResponse {
    fn default() -> Self {
        BasicSrvResponse {
            resp: ::std::string::String::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for BasicSrvResponse {}


pub struct BasicSrv;
