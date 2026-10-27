use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetSpeedSliderFractionRequest {
    pub speed_slider_fraction: f64,
}

impl Default for SetSpeedSliderFractionRequest {
    fn default() -> Self {
        SetSpeedSliderFractionRequest {
            speed_slider_fraction: 0.0,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for SetSpeedSliderFractionRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetSpeedSliderFractionResponse {
    pub success: bool,
}

impl Default for SetSpeedSliderFractionResponse {
    fn default() -> Self {
        SetSpeedSliderFractionResponse {
            success: false,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for SetSpeedSliderFractionResponse {}


pub struct SetSpeedSliderFraction;
