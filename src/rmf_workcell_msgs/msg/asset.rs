use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Asset {
    pub guid: ::std::string::String,
    #[serde(rename = "type")]    pub type_: ::std::string::String,
}

impl Default for Asset {
    fn default() -> Self {
        Asset {
            guid: ::std::string::String::new(),
            type_: ::std::string::String::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for Asset {}
