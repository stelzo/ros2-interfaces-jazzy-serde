use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RelocalizeFromGNSSRequest {

}

impl Default for RelocalizeFromGNSSRequest {
    fn default() -> Self {
        RelocalizeFromGNSSRequest {

        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for RelocalizeFromGNSSRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RelocalizeFromGNSSResponse {
    pub accepted: bool,
}

impl Default for RelocalizeFromGNSSResponse {
    fn default() -> Self {
        RelocalizeFromGNSSResponse {
            accepted: false,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for RelocalizeFromGNSSResponse {}


pub struct RelocalizeFromGNSS;
