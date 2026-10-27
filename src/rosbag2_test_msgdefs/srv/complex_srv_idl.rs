use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComplexSrvIdlRequest {
    pub req: crate::rosbag2_test_msgdefs::msg::BasicIdl,
}

impl Default for ComplexSrvIdlRequest {
    fn default() -> Self {
        ComplexSrvIdlRequest {
            req: crate::rosbag2_test_msgdefs::msg::BasicIdl::default(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for ComplexSrvIdlRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComplexSrvIdlResponse {
    pub resp: crate::rosbag2_test_msgdefs::msg::BasicIdl,
}

impl Default for ComplexSrvIdlResponse {
    fn default() -> Self {
        ComplexSrvIdlResponse {
            resp: crate::rosbag2_test_msgdefs::msg::BasicIdl::default(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for ComplexSrvIdlResponse {}


pub struct ComplexSrvIdl;
