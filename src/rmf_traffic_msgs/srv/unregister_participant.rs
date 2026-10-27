use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnregisterParticipantRequest {
    pub participant_id: u64,
}

impl Default for UnregisterParticipantRequest {
    fn default() -> Self {
        UnregisterParticipantRequest {
            participant_id: 0,
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for UnregisterParticipantRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnregisterParticipantResponse {
    pub confirmation: bool,
    pub error: ::std::string::String,
}

impl Default for UnregisterParticipantResponse {
    fn default() -> Self {
        UnregisterParticipantResponse {
            confirmation: false,
            error: ::std::string::String::new(),
        }
    }
}

#[cfg(feature = "ros2-client")]
impl ros2_client::Message for UnregisterParticipantResponse {}


pub struct UnregisterParticipant;
