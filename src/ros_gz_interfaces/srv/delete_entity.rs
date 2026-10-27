use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteEntityRequest {
    pub entity: crate::ros_gz_interfaces::msg::Entity,
}

impl Default for DeleteEntityRequest {
    fn default() -> Self {
        DeleteEntityRequest {
            entity: crate::ros_gz_interfaces::msg::Entity::default(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for DeleteEntityRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteEntityResponse {
    pub success: bool,
}

impl Default for DeleteEntityResponse {
    fn default() -> Self {
        DeleteEntityResponse {
            success: false,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for DeleteEntityResponse {}


pub struct DeleteEntity;
