use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListControllerTypesRequest {

}

impl Default for ListControllerTypesRequest {
    fn default() -> Self {
        ListControllerTypesRequest {

        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for ListControllerTypesRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListControllerTypesResponse {
    pub types: Vec<::std::string::String>,
    pub base_classes: Vec<::std::string::String>,
}

impl Default for ListControllerTypesResponse {
    fn default() -> Self {
        ListControllerTypesResponse {
            types: Vec::new(),
            base_classes: Vec::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for ListControllerTypesResponse {}


pub struct ListControllerTypes;
