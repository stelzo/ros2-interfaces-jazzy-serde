use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReloadControllerLibrariesRequest {
    pub force_kill: bool,
}

impl Default for ReloadControllerLibrariesRequest {
    fn default() -> Self {
        ReloadControllerLibrariesRequest {
            force_kill: false,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for ReloadControllerLibrariesRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReloadControllerLibrariesResponse {
    pub ok: bool,
}

impl Default for ReloadControllerLibrariesResponse {
    fn default() -> Self {
        ReloadControllerLibrariesResponse {
            ok: false,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for ReloadControllerLibrariesResponse {}


pub struct ReloadControllerLibraries;
