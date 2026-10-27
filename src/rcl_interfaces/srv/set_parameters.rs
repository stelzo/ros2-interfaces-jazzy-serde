use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetParametersRequest {
    pub parameters: Vec<crate::rcl_interfaces::msg::Parameter>,
}

impl Default for SetParametersRequest {
    fn default() -> Self {
        SetParametersRequest {
            parameters: Vec::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for SetParametersRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetParametersResponse {
    pub results: Vec<crate::rcl_interfaces::msg::SetParametersResult>,
}

impl Default for SetParametersResponse {
    fn default() -> Self {
        SetParametersResponse {
            results: Vec::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for SetParametersResponse {}


pub struct SetParameters;
