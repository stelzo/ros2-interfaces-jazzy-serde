use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SbgGpsRaw {
    pub header: crate::std_msgs::msg::Header,
    pub data: Vec<u8>,
}

impl Default for SbgGpsRaw {
    fn default() -> Self {
        SbgGpsRaw {
            header: crate::std_msgs::msg::Header::default(),
            data: Vec::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for SbgGpsRaw {}
