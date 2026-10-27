use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GimbalGetInformationRequest {

}

impl Default for GimbalGetInformationRequest {
    fn default() -> Self {
        GimbalGetInformationRequest {

        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for GimbalGetInformationRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GimbalGetInformationResponse {
    pub success: bool,
    pub result: u8,
}

impl Default for GimbalGetInformationResponse {
    fn default() -> Self {
        GimbalGetInformationResponse {
            success: false,
            result: 0,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for GimbalGetInformationResponse {}


pub struct GimbalGetInformation;
