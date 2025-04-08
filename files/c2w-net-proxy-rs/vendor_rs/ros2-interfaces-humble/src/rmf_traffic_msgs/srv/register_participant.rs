use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RegisterParticipantRequest {
    pub description: crate::rmf_traffic_msgs::msg::ParticipantDescription,
}

impl Default for RegisterParticipantRequest {
    fn default() -> Self {
        RegisterParticipantRequest {
            description: crate::rmf_traffic_msgs::msg::ParticipantDescription::default(),
        }
    }
}

impl ros2_client::Message for RegisterParticipantRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RegisterParticipantResponse {
    pub participant_id: u64,
    pub last_itinerary_version: u64,
    pub last_plan_id: u64,
    pub next_storage_base: u64,
    pub error: ::std::string::String,
}

impl Default for RegisterParticipantResponse {
    fn default() -> Self {
        RegisterParticipantResponse {
            participant_id: 0,
            last_itinerary_version: 0,
            last_plan_id: 0,
            next_storage_base: 0,
            error: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for RegisterParticipantResponse {}


pub struct RegisterParticipant;
impl ros2_client::Service for RegisterParticipant {
    type Request = RegisterParticipantRequest;
    type Response = RegisterParticipantResponse;

    fn request_type_name(&self) -> &str { "RegisterParticipantRequest" }
    fn response_type_name(&self) -> &str { "RegisterParticipantResponse" }
}
