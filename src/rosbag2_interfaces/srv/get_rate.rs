use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetRateRequest {

}

impl Default for GetRateRequest {
    fn default() -> Self {
        GetRateRequest {

        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for GetRateRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetRateResponse {
    pub rate: f64,
}

impl Default for GetRateResponse {
    fn default() -> Self {
        GetRateResponse {
            rate: 0.0,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for GetRateResponse {}


pub struct GetRate;
