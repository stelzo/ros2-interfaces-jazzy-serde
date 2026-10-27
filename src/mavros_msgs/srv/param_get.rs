use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParamGetRequest {
    pub param_id: ::std::string::String,
}

impl Default for ParamGetRequest {
    fn default() -> Self {
        ParamGetRequest {
            param_id: ::std::string::String::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for ParamGetRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParamGetResponse {
    pub success: bool,
    pub value: crate::mavros_msgs::msg::ParamValue,
}

impl Default for ParamGetResponse {
    fn default() -> Self {
        ParamGetResponse {
            success: false,
            value: crate::mavros_msgs::msg::ParamValue::default(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for ParamGetResponse {}


pub struct ParamGet;
