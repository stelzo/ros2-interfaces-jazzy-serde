use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReloadDockDatabaseRequest {
    pub filepath: ::std::string::String,
}

impl Default for ReloadDockDatabaseRequest {
    fn default() -> Self {
        ReloadDockDatabaseRequest {
            filepath: ::std::string::String::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for ReloadDockDatabaseRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReloadDockDatabaseResponse {
    pub success: bool,
}

impl Default for ReloadDockDatabaseResponse {
    fn default() -> Self {
        ReloadDockDatabaseResponse {
            success: false,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for ReloadDockDatabaseResponse {}


pub struct ReloadDockDatabase;
