use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteLoadCarriersRequest {
    pub load_carrier_ids: Vec<::std::string::String>,
}

impl Default for DeleteLoadCarriersRequest {
    fn default() -> Self {
        DeleteLoadCarriersRequest {
            load_carrier_ids: Vec::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for DeleteLoadCarriersRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteLoadCarriersResponse {
    pub return_code: crate::rc_common_msgs::msg::ReturnCode,
}

impl Default for DeleteLoadCarriersResponse {
    fn default() -> Self {
        DeleteLoadCarriersResponse {
            return_code: crate::rc_common_msgs::msg::ReturnCode::default(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for DeleteLoadCarriersResponse {}


pub struct DeleteLoadCarriers;
