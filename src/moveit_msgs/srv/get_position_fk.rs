use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetPositionFKRequest {
    pub header: crate::std_msgs::msg::Header,
    pub fk_link_names: Vec<::std::string::String>,
    pub robot_state: crate::moveit_msgs::msg::RobotState,
}

impl Default for GetPositionFKRequest {
    fn default() -> Self {
        GetPositionFKRequest {
            header: crate::std_msgs::msg::Header::default(),
            fk_link_names: Vec::new(),
            robot_state: crate::moveit_msgs::msg::RobotState::default(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for GetPositionFKRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetPositionFKResponse {
    pub pose_stamped: Vec<crate::geometry_msgs::msg::PoseStamped>,
    pub fk_link_names: Vec<::std::string::String>,
    pub error_code: crate::moveit_msgs::msg::MoveItErrorCodes,
}

impl Default for GetPositionFKResponse {
    fn default() -> Self {
        GetPositionFKResponse {
            pose_stamped: Vec::new(),
            fk_link_names: Vec::new(),
            error_code: crate::moveit_msgs::msg::MoveItErrorCodes::default(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for GetPositionFKResponse {}


pub struct GetPositionFK;
