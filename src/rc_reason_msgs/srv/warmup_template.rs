use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WarmupTemplateRequest {
    pub template_id: ::std::string::String,
}

impl Default for WarmupTemplateRequest {
    fn default() -> Self {
        WarmupTemplateRequest {
            template_id: ::std::string::String::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for WarmupTemplateRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WarmupTemplateResponse {
    pub return_code: crate::rc_common_msgs::msg::ReturnCode,
}

impl Default for WarmupTemplateResponse {
    fn default() -> Self {
        WarmupTemplateResponse {
            return_code: crate::rc_common_msgs::msg::ReturnCode::default(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for WarmupTemplateResponse {}


pub struct WarmupTemplate;
