use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SickScanExitSrvRequest {

}

impl Default for SickScanExitSrvRequest {
    fn default() -> Self {
        SickScanExitSrvRequest {

        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for SickScanExitSrvRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SickScanExitSrvResponse {
    pub success: bool,
}

impl Default for SickScanExitSrvResponse {
    fn default() -> Self {
        SickScanExitSrvResponse {
            success: false,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for SickScanExitSrvResponse {}


pub struct SickScanExitSrv;
