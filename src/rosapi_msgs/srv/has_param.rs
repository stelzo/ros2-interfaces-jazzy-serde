use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HasParamRequest {
    pub name: ::std::string::String,
}

impl Default for HasParamRequest {
    fn default() -> Self {
        HasParamRequest {
            name: ::std::string::String::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for HasParamRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HasParamResponse {
    pub exists: bool,
}

impl Default for HasParamResponse {
    fn default() -> Self {
        HasParamResponse {
            exists: false,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for HasParamResponse {}


pub struct HasParam;
