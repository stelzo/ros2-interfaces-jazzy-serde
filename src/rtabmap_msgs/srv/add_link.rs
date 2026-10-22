use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddLinkRequest {
    pub link: crate::rtabmap_msgs::msg::Link,
}

impl Default for AddLinkRequest {
    fn default() -> Self {
        AddLinkRequest {
            link: crate::rtabmap_msgs::msg::Link::default(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for AddLinkRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddLinkResponse {

}

impl Default for AddLinkResponse {
    fn default() -> Self {
        AddLinkResponse {

        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for AddLinkResponse {}


pub struct AddLink;
#[cfg(feature = "ros2-client")]
impl ros2_client::Service for AddLink {
    type Request = AddLinkRequest;
    type Response = AddLinkResponse;

    fn request_type_name(&self) -> &str { "AddLinkRequest" }
    fn response_type_name(&self) -> &str { "AddLinkResponse" }
}
