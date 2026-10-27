use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListParametersRequest {
    pub prefixes: Vec<::std::string::String>,
    pub depth: u64,
}

impl ListParametersRequest {
    pub const DEPTH_RECURSIVE: u64 = 0;
}

impl Default for ListParametersRequest {
    fn default() -> Self {
        ListParametersRequest {
            prefixes: Vec::new(),
            depth: 0,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for ListParametersRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListParametersResponse {
    pub result: crate::rcl_interfaces::msg::ListParametersResult,
}

impl Default for ListParametersResponse {
    fn default() -> Self {
        ListParametersResponse {
            result: crate::rcl_interfaces::msg::ListParametersResult::default(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for ListParametersResponse {}


pub struct ListParameters;
