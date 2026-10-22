use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Synchronize {

}

impl Default for Synchronize {
    fn default() -> Self {
        Synchronize {

        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for Synchronize {}
