use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileRenameRequest {
    pub old_path: ::std::string::String,
    pub new_path: ::std::string::String,
}

impl Default for FileRenameRequest {
    fn default() -> Self {
        FileRenameRequest {
            old_path: ::std::string::String::new(),
            new_path: ::std::string::String::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for FileRenameRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileRenameResponse {
    pub success: bool,
    pub r_errno: i32,
}

impl Default for FileRenameResponse {
    fn default() -> Self {
        FileRenameResponse {
            success: false,
            r_errno: 0,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for FileRenameResponse {}


pub struct FileRename;
