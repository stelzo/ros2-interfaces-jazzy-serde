use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetRobotSoftwareVersionRequest {

}

impl Default for GetRobotSoftwareVersionRequest {
    fn default() -> Self {
        GetRobotSoftwareVersionRequest {

        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for GetRobotSoftwareVersionRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetRobotSoftwareVersionResponse {
    pub major: u32,
    pub minor: u32,
    pub bugfix: u32,
    pub build: u32,
}

impl Default for GetRobotSoftwareVersionResponse {
    fn default() -> Self {
        GetRobotSoftwareVersionResponse {
            major: 0,
            minor: 0,
            bugfix: 0,
            build: 0,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for GetRobotSoftwareVersionResponse {}


pub struct GetRobotSoftwareVersion;
