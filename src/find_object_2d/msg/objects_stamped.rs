use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ObjectsStamped {
    pub header: crate::std_msgs::msg::Header,
    pub objects: crate::std_msgs::msg::Float32MultiArray,
}

impl Default for ObjectsStamped {
    fn default() -> Self {
        ObjectsStamped {
            header: crate::std_msgs::msg::Header::default(),
            objects: crate::std_msgs::msg::Float32MultiArray::default(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for ObjectsStamped {}
