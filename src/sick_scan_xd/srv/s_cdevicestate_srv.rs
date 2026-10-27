use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SCdevicestateSrvRequest {

}

impl Default for SCdevicestateSrvRequest {
    fn default() -> Self {
        SCdevicestateSrvRequest {

        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for SCdevicestateSrvRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SCdevicestateSrvResponse {
    pub state: i32,
    pub success: bool,
}

impl Default for SCdevicestateSrvResponse {
    fn default() -> Self {
        SCdevicestateSrvResponse {
            state: 0,
            success: false,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for SCdevicestateSrvResponse {}


pub struct SCdevicestateSrv;
