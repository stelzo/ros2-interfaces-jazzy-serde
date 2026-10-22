use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileRemoveRequest {
    pub file_path: ::std::string::String,
}

impl Default for FileRemoveRequest {
    fn default() -> Self {
        FileRemoveRequest {
            file_path: ::std::string::String::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for FileRemoveRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileRemoveResponse {
    pub success: bool,
    pub r_errno: i32,
}

impl Default for FileRemoveResponse {
    fn default() -> Self {
        FileRemoveResponse {
            success: false,
            r_errno: 0,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for FileRemoveResponse {}


pub struct FileRemove;
#[cfg(feature = "ros2-client")]
impl ros2_client::Service for FileRemove {
    type Request = FileRemoveRequest;
    type Response = FileRemoveResponse;

    fn request_type_name(&self) -> &str { "FileRemoveRequest" }
    fn response_type_name(&self) -> &str { "FileRemoveResponse" }
}
