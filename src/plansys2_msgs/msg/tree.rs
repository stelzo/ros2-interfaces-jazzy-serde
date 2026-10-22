use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tree {
    pub nodes: Vec<crate::plansys2_msgs::msg::Node>,
}

impl Default for Tree {
    fn default() -> Self {
        Tree {
            nodes: Vec::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for Tree {}
