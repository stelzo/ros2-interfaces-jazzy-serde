use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComplexSrvMsgRequest {
    pub req: crate::rosbag2_test_msgdefs::msg::BasicMsg,
}

impl Default for ComplexSrvMsgRequest {
    fn default() -> Self {
        ComplexSrvMsgRequest {
            req: crate::rosbag2_test_msgdefs::msg::BasicMsg::default(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for ComplexSrvMsgRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComplexSrvMsgResponse {
    pub resp: crate::rosbag2_test_msgdefs::msg::BasicMsg,
}

impl Default for ComplexSrvMsgResponse {
    fn default() -> Self {
        ComplexSrvMsgResponse {
            resp: crate::rosbag2_test_msgdefs::msg::BasicMsg::default(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for ComplexSrvMsgResponse {}


pub struct ComplexSrvMsg;
