use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BoundingBoxQueryRequest {
    pub min: crate::geometry_msgs::msg::Point,
    pub max: crate::geometry_msgs::msg::Point,
}

impl Default for BoundingBoxQueryRequest {
    fn default() -> Self {
        BoundingBoxQueryRequest {
            min: crate::geometry_msgs::msg::Point::default(),
            max: crate::geometry_msgs::msg::Point::default(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for BoundingBoxQueryRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BoundingBoxQueryResponse {

}

impl Default for BoundingBoxQueryResponse {
    fn default() -> Self {
        BoundingBoxQueryResponse {

        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for BoundingBoxQueryResponse {}


pub struct BoundingBoxQuery;
