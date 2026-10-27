use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ECRChangeArrSrvRequest {
    pub active: bool,
}

impl Default for ECRChangeArrSrvRequest {
    fn default() -> Self {
        ECRChangeArrSrvRequest {
            active: false,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for ECRChangeArrSrvRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ECRChangeArrSrvResponse {
    pub success: bool,
}

impl Default for ECRChangeArrSrvResponse {
    fn default() -> Self {
        ECRChangeArrSrvResponse {
            success: false,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for ECRChangeArrSrvResponse {}


pub struct ECRChangeArrSrv;
