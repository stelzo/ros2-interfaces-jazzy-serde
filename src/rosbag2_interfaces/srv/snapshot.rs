use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SnapshotRequest {

}

impl Default for SnapshotRequest {
    fn default() -> Self {
        SnapshotRequest {

        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for SnapshotRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SnapshotResponse {
    pub success: bool,
}

impl Default for SnapshotResponse {
    fn default() -> Self {
        SnapshotResponse {
            success: false,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for SnapshotResponse {}


pub struct Snapshot;
