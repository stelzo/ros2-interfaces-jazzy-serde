use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ManageLifecycleNodesRequest {
    pub command: u8,
}

impl ManageLifecycleNodesRequest {
    pub const STARTUP: u8 = 0;
    pub const PAUSE: u8 = 1;
    pub const RESUME: u8 = 2;
    pub const RESET: u8 = 3;
    pub const SHUTDOWN: u8 = 4;
    pub const CONFIGURE: u8 = 5;
    pub const CLEANUP: u8 = 6;
}

impl Default for ManageLifecycleNodesRequest {
    fn default() -> Self {
        ManageLifecycleNodesRequest {
            command: 0,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for ManageLifecycleNodesRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ManageLifecycleNodesResponse {
    pub success: bool,
}

impl Default for ManageLifecycleNodesResponse {
    fn default() -> Self {
        ManageLifecycleNodesResponse {
            success: false,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for ManageLifecycleNodesResponse {}


pub struct ManageLifecycleNodes;
