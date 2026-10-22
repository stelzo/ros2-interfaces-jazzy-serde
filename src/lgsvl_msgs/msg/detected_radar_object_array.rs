use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DetectedRadarObjectArray {
    pub header: crate::std_msgs::msg::Header,
    pub objects: Vec<crate::lgsvl_msgs::msg::DetectedRadarObject>,
}

impl Default for DetectedRadarObjectArray {
    fn default() -> Self {
        DetectedRadarObjectArray {
            header: crate::std_msgs::msg::Header::default(),
            objects: Vec::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for DetectedRadarObjectArray {}
