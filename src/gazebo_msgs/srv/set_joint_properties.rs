use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetJointPropertiesRequest {
    pub joint_name: ::std::string::String,
    pub ode_joint_config: crate::gazebo_msgs::msg::ODEJointProperties,
}

impl Default for SetJointPropertiesRequest {
    fn default() -> Self {
        SetJointPropertiesRequest {
            joint_name: ::std::string::String::new(),
            ode_joint_config: crate::gazebo_msgs::msg::ODEJointProperties::default(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for SetJointPropertiesRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetJointPropertiesResponse {
    pub success: bool,
    pub status_message: ::std::string::String,
}

impl Default for SetJointPropertiesResponse {
    fn default() -> Self {
        SetJointPropertiesResponse {
            success: false,
            status_message: ::std::string::String::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for SetJointPropertiesResponse {}


pub struct SetJointProperties;
