use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetLoadCarrierRequest {
    pub load_carrier: crate::rc_reason_msgs::msg::LoadCarrierModel,
}

impl Default for SetLoadCarrierRequest {
    fn default() -> Self {
        SetLoadCarrierRequest {
            load_carrier: crate::rc_reason_msgs::msg::LoadCarrierModel::default(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for SetLoadCarrierRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetLoadCarrierResponse {
    pub return_code: crate::rc_common_msgs::msg::ReturnCode,
}

impl Default for SetLoadCarrierResponse {
    fn default() -> Self {
        SetLoadCarrierResponse {
            return_code: crate::rc_common_msgs::msg::ReturnCode::default(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for SetLoadCarrierResponse {}


pub struct SetLoadCarrier;
