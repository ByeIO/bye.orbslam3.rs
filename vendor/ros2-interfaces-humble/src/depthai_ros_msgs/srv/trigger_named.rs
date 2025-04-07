use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TriggerNamedRequest {
    pub name: ::std::string::String,
}

impl Default for TriggerNamedRequest {
    fn default() -> Self {
        TriggerNamedRequest {
            name: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for TriggerNamedRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TriggerNamedResponse {
    pub success: bool,
    pub message: ::std::string::String,
}

impl Default for TriggerNamedResponse {
    fn default() -> Self {
        TriggerNamedResponse {
            success: false,
            message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for TriggerNamedResponse {}


pub struct TriggerNamed;
impl ros2_client::Service for TriggerNamed {
    type Request = TriggerNamedRequest;
    type Response = TriggerNamedResponse;

    fn request_type_name(&self) -> &str { "TriggerNamedRequest" }
    fn response_type_name(&self) -> &str { "TriggerNamedResponse" }
}
