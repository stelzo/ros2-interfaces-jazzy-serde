use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListSchedulesRequest {
    pub created_after: i64,
}

impl Default for ListSchedulesRequest {
    fn default() -> Self {
        ListSchedulesRequest {
            created_after: 0,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for ListSchedulesRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListSchedulesResponse {
    pub success: bool,
    pub message: ::std::string::String,
    pub schedules: Vec<crate::rmf_scheduler_msgs::msg::Schedule>,
}

impl Default for ListSchedulesResponse {
    fn default() -> Self {
        ListSchedulesResponse {
            success: false,
            message: ::std::string::String::new(),
            schedules: Vec::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for ListSchedulesResponse {}


pub struct ListSchedules;
