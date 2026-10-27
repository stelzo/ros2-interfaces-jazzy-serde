use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApplyLinkWrenchRequest {
    pub link_name: ::std::string::String,
    pub reference_frame: ::std::string::String,
    pub reference_point: crate::geometry_msgs::msg::Point,
    pub wrench: crate::geometry_msgs::msg::Wrench,
    pub start_time: crate::builtin_interfaces::msg::Time,
    pub duration: crate::builtin_interfaces::msg::Duration,
}

impl Default for ApplyLinkWrenchRequest {
    fn default() -> Self {
        ApplyLinkWrenchRequest {
            link_name: ::std::string::String::new(),
            reference_frame: ::std::string::String::new(),
            reference_point: crate::geometry_msgs::msg::Point::default(),
            wrench: crate::geometry_msgs::msg::Wrench::default(),
            start_time: crate::builtin_interfaces::msg::Time::default(),
            duration: crate::builtin_interfaces::msg::Duration::default(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for ApplyLinkWrenchRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApplyLinkWrenchResponse {
    pub success: bool,
    pub status_message: ::std::string::String,
}

impl Default for ApplyLinkWrenchResponse {
    fn default() -> Self {
        ApplyLinkWrenchResponse {
            success: false,
            status_message: ::std::string::String::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for ApplyLinkWrenchResponse {}


pub struct ApplyLinkWrench;
