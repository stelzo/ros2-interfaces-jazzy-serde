use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetTimeRequest {

}

impl Default for GetTimeRequest {
    fn default() -> Self {
        GetTimeRequest {

        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for GetTimeRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetTimeResponse {
    pub time: crate::builtin_interfaces::msg::Time,
}

impl Default for GetTimeResponse {
    fn default() -> Self {
        GetTimeResponse {
            time: crate::builtin_interfaces::msg::Time::default(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for GetTimeResponse {}


pub struct GetTime;
