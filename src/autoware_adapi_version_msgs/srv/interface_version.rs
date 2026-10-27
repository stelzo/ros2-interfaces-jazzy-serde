use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InterfaceVersionRequest {

}

impl Default for InterfaceVersionRequest {
    fn default() -> Self {
        InterfaceVersionRequest {

        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for InterfaceVersionRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InterfaceVersionResponse {
    pub major: u16,
    pub minor: u16,
    pub patch: u16,
}

impl Default for InterfaceVersionResponse {
    fn default() -> Self {
        InterfaceVersionResponse {
            major: 0,
            minor: 0,
            patch: 0,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for InterfaceVersionResponse {}


pub struct InterfaceVersion;
