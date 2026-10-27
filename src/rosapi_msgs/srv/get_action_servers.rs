use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetActionServersRequest {

}

impl Default for GetActionServersRequest {
    fn default() -> Self {
        GetActionServersRequest {

        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for GetActionServersRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetActionServersResponse {
    pub action_servers: Vec<::std::string::String>,
}

impl Default for GetActionServersResponse {
    fn default() -> Self {
        GetActionServersResponse {
            action_servers: Vec::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for GetActionServersResponse {}


pub struct GetActionServers;
