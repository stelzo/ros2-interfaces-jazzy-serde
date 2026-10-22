use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Empty {

}

impl Default for Empty {
    fn default() -> Self {
        Empty {

        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for Empty {}
