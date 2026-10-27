use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClearCostmapAroundRobotRequest {
    pub reset_distance: f32,
}

impl Default for ClearCostmapAroundRobotRequest {
    fn default() -> Self {
        ClearCostmapAroundRobotRequest {
            reset_distance: 0.0,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for ClearCostmapAroundRobotRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClearCostmapAroundRobotResponse {
    pub response: crate::std_msgs::msg::Empty,
}

impl Default for ClearCostmapAroundRobotResponse {
    fn default() -> Self {
        ClearCostmapAroundRobotResponse {
            response: crate::std_msgs::msg::Empty::default(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for ClearCostmapAroundRobotResponse {}


pub struct ClearCostmapAroundRobot;
