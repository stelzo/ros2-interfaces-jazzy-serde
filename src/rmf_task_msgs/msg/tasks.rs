use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tasks {
    pub tasks: Vec<crate::rmf_task_msgs::msg::TaskSummary>,
}

impl Default for Tasks {
    fn default() -> Self {
        Tasks {
            tasks: Vec::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for Tasks {}
