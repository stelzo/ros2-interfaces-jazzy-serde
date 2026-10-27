use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FieldDataRequest {

}

impl Default for FieldDataRequest {
    fn default() -> Self {
        FieldDataRequest {

        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for FieldDataRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FieldDataResponse {
    pub fields: Vec<crate::sick_safetyscanners2_interfaces::msg::Field>,
    pub device_name: ::std::string::String,
    pub monitoring_cases: Vec<crate::sick_safetyscanners2_interfaces::msg::MonitoringCase>,
}

impl Default for FieldDataResponse {
    fn default() -> Self {
        FieldDataResponse {
            fields: Vec::new(),
            device_name: ::std::string::String::new(),
            monitoring_cases: Vec::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for FieldDataResponse {}


pub struct FieldData;
