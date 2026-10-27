use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateFilenameRequest {
    pub filename: ::std::string::String,
}

impl Default for UpdateFilenameRequest {
    fn default() -> Self {
        UpdateFilenameRequest {
            filename: ::std::string::String::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for UpdateFilenameRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateFilenameResponse {
    pub success: bool,
}

impl Default for UpdateFilenameResponse {
    fn default() -> Self {
        UpdateFilenameResponse {
            success: false,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for UpdateFilenameResponse {}


pub struct UpdateFilename;
