use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetExposureRequest {
    pub auto_exposure: bool,
    pub time: i64,
}

impl Default for SetExposureRequest {
    fn default() -> Self {
        SetExposureRequest {
            auto_exposure: false,
            time: 0,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for SetExposureRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetExposureResponse {
    pub auto_exposure: bool,
    pub time: i64,
}

impl Default for SetExposureResponse {
    fn default() -> Self {
        SetExposureResponse {
            auto_exposure: false,
            time: 0,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for SetExposureResponse {}


pub struct SetExposure;
