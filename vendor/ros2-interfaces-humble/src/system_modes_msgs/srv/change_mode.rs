use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChangeModeRequest {
    pub mode_name: ::std::string::String,
}

impl Default for ChangeModeRequest {
    fn default() -> Self {
        ChangeModeRequest {
            mode_name: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for ChangeModeRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChangeModeResponse {
    pub success: bool,
}

impl Default for ChangeModeResponse {
    fn default() -> Self {
        ChangeModeResponse {
            success: false,
        }
    }
}

impl ros2_client::Message for ChangeModeResponse {}


pub struct ChangeMode;
impl ros2_client::Service for ChangeMode {
    type Request = ChangeModeRequest;
    type Response = ChangeModeResponse;

    fn request_type_name(&self) -> &str { "ChangeModeRequest" }
    fn response_type_name(&self) -> &str { "ChangeModeResponse" }
}
