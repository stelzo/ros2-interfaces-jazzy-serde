use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileCloseRequest {
    pub file_path: ::std::string::String,
}

impl Default for FileCloseRequest {
    fn default() -> Self {
        FileCloseRequest {
            file_path: ::std::string::String::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for FileCloseRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileCloseResponse {
    pub success: bool,
    pub r_errno: i32,
}

impl Default for FileCloseResponse {
    fn default() -> Self {
        FileCloseResponse {
            success: false,
            r_errno: 0,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for FileCloseResponse {}


pub struct FileClose;
