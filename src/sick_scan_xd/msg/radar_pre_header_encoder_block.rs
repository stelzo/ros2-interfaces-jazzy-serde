use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RadarPreHeaderEncoderBlock {
    pub udiencoderpos: u32,
    pub iencoderspeed: i16,
}

impl Default for RadarPreHeaderEncoderBlock {
    fn default() -> Self {
        RadarPreHeaderEncoderBlock {
            udiencoderpos: 0,
            iencoderspeed: 0,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for RadarPreHeaderEncoderBlock {}
