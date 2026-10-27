use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetOctomapRequest {

}

impl Default for GetOctomapRequest {
    fn default() -> Self {
        GetOctomapRequest {

        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for GetOctomapRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetOctomapResponse {
    pub map: crate::octomap_msgs::msg::Octomap,
}

impl Default for GetOctomapResponse {
    fn default() -> Self {
        GetOctomapResponse {
            map: crate::octomap_msgs::msg::Octomap::default(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for GetOctomapResponse {}


pub struct GetOctomap;
