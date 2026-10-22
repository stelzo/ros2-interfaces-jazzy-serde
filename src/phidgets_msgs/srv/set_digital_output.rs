use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetDigitalOutputRequest {
    pub index: u16,
    pub state: bool,
}

impl Default for SetDigitalOutputRequest {
    fn default() -> Self {
        SetDigitalOutputRequest {
            index: 0,
            state: false,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for SetDigitalOutputRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetDigitalOutputResponse {
    pub success: bool,
}

impl Default for SetDigitalOutputResponse {
    fn default() -> Self {
        SetDigitalOutputResponse {
            success: false,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for SetDigitalOutputResponse {}


pub struct SetDigitalOutput;
#[cfg(feature = "ros2-client")]
impl ros2_client::Service for SetDigitalOutput {
    type Request = SetDigitalOutputRequest;
    type Response = SetDigitalOutputResponse;

    fn request_type_name(&self) -> &str { "SetDigitalOutputRequest" }
    fn response_type_name(&self) -> &str { "SetDigitalOutputResponse" }
}
