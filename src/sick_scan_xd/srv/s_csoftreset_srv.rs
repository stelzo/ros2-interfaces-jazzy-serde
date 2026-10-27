use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SCsoftresetSrvRequest {

}

impl Default for SCsoftresetSrvRequest {
    fn default() -> Self {
        SCsoftresetSrvRequest {

        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for SCsoftresetSrvRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SCsoftresetSrvResponse {
    pub success: bool,
}

impl Default for SCsoftresetSrvResponse {
    fn default() -> Self {
        SCsoftresetSrvResponse {
            success: false,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for SCsoftresetSrvResponse {}


pub struct SCsoftresetSrv;
