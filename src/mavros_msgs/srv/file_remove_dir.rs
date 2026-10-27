use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileRemoveDirRequest {
    pub dir_path: ::std::string::String,
}

impl Default for FileRemoveDirRequest {
    fn default() -> Self {
        FileRemoveDirRequest {
            dir_path: ::std::string::String::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for FileRemoveDirRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileRemoveDirResponse {
    pub success: bool,
    pub r_errno: i32,
}

impl Default for FileRemoveDirResponse {
    fn default() -> Self {
        FileRemoveDirResponse {
            success: false,
            r_errno: 0,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for FileRemoveDirResponse {}


pub struct FileRemoveDir;
