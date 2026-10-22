use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ball {
    pub center: crate::rcss3d_agent_msgs::msg::Spherical,
}

impl Default for Ball {
    fn default() -> Self {
        Ball {
            center: crate::rcss3d_agent_msgs::msg::Spherical::default(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for Ball {}
