use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ValidateDomainRequest {
    pub domain: ::std::string::String,
}

impl Default for ValidateDomainRequest {
    fn default() -> Self {
        ValidateDomainRequest {
            domain: ::std::string::String::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for ValidateDomainRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ValidateDomainResponse {
    pub success: bool,
    pub error_info: ::std::string::String,
}

impl Default for ValidateDomainResponse {
    fn default() -> Self {
        ValidateDomainResponse {
            success: false,
            error_info: ::std::string::String::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for ValidateDomainResponse {}


pub struct ValidateDomain;
