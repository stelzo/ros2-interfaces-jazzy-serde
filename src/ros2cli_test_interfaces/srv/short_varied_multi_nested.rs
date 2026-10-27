use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShortVariedMultiNestedRequest {
    pub short_varied_nested: crate::ros2cli_test_interfaces::msg::ShortVariedNested,
}

impl Default for ShortVariedMultiNestedRequest {
    fn default() -> Self {
        ShortVariedMultiNestedRequest {
            short_varied_nested: crate::ros2cli_test_interfaces::msg::ShortVariedNested::default(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for ShortVariedMultiNestedRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShortVariedMultiNestedResponse {
    pub bool_value: bool,
}

impl Default for ShortVariedMultiNestedResponse {
    fn default() -> Self {
        ShortVariedMultiNestedResponse {
            bool_value: false,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for ShortVariedMultiNestedResponse {}


pub struct ShortVariedMultiNested;
