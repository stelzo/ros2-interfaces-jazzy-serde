use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetObjectInformationRequest {
    #[serde(rename = "type")]    pub type_: crate::object_recognition_msgs::msg::ObjectType,
}

impl Default for GetObjectInformationRequest {
    fn default() -> Self {
        GetObjectInformationRequest {
            type_: crate::object_recognition_msgs::msg::ObjectType::default(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for GetObjectInformationRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetObjectInformationResponse {
    pub information: crate::object_recognition_msgs::msg::ObjectInformation,
}

impl Default for GetObjectInformationResponse {
    fn default() -> Self {
        GetObjectInformationResponse {
            information: crate::object_recognition_msgs::msg::ObjectInformation::default(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for GetObjectInformationResponse {}


pub struct GetObjectInformation;
