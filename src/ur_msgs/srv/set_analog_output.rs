use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetAnalogOutputRequest {
    pub data: crate::ur_msgs::msg::Analog,
}

impl Default for SetAnalogOutputRequest {
    fn default() -> Self {
        SetAnalogOutputRequest {
            data: crate::ur_msgs::msg::Analog::default(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for SetAnalogOutputRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetAnalogOutputResponse {
    pub success: bool,
}

impl Default for SetAnalogOutputResponse {
    fn default() -> Self {
        SetAnalogOutputResponse {
            success: false,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for SetAnalogOutputResponse {}


pub struct SetAnalogOutput;
