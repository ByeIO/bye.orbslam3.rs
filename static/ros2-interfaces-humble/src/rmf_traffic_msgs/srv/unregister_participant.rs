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

impl ros2_client::Message for UnregisterParticipantResponse {}


pub struct UnregisterParticipant;
impl ros2_client::Service for UnregisterParticipant {
    type Request = UnregisterParticipantRequest;
    type Response = UnregisterParticipantResponse;

    fn request_type_name(&self) -> &str { "UnregisterParticipantRequest" }
    fn response_type_name(&self) -> &str { "UnregisterParticipantResponse" }
}
