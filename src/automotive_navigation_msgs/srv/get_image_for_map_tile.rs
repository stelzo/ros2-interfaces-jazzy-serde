use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetImageForMapTileRequest {
    pub tile_id: ::std::string::String,
}

impl Default for GetImageForMapTileRequest {
    fn default() -> Self {
        GetImageForMapTileRequest {
            tile_id: ::std::string::String::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for GetImageForMapTileRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetImageForMapTileResponse {
    pub success: bool,
    pub message: ::std::string::String,
}

impl Default for GetImageForMapTileResponse {
    fn default() -> Self {
        GetImageForMapTileResponse {
            success: false,
            message: ::std::string::String::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for GetImageForMapTileResponse {}


pub struct GetImageForMapTile;
