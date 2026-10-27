use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteRegionsOfInterest3DRequest {
    pub region_of_interest_ids: Vec<::std::string::String>,
}

impl Default for DeleteRegionsOfInterest3DRequest {
    fn default() -> Self {
        DeleteRegionsOfInterest3DRequest {
            region_of_interest_ids: Vec::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for DeleteRegionsOfInterest3DRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteRegionsOfInterest3DResponse {
    pub return_code: crate::rc_common_msgs::msg::ReturnCode,
}

impl Default for DeleteRegionsOfInterest3DResponse {
    fn default() -> Self {
        DeleteRegionsOfInterest3DResponse {
            return_code: crate::rc_common_msgs::msg::ReturnCode::default(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for DeleteRegionsOfInterest3DResponse {}


pub struct DeleteRegionsOfInterest3D;
