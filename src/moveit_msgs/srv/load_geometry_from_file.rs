use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoadGeometryFromFileRequest {
    pub file_path_and_name: ::std::string::String,
}

impl Default for LoadGeometryFromFileRequest {
    fn default() -> Self {
        LoadGeometryFromFileRequest {
            file_path_and_name: ::std::string::String::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for LoadGeometryFromFileRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoadGeometryFromFileResponse {
    pub success: bool,
}

impl Default for LoadGeometryFromFileResponse {
    fn default() -> Self {
        LoadGeometryFromFileResponse {
            success: false,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for LoadGeometryFromFileResponse {}


pub struct LoadGeometryFromFile;
