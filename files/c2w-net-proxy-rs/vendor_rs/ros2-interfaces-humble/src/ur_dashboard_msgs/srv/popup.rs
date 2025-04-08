use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PopupRequest {
    pub message: ::std::string::String,
}

impl Default for PopupRequest {
    fn default() -> Self {
        PopupRequest {
            message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for PopupRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PopupResponse {
    pub answer: ::std::string::String,
    pub success: bool,
}

impl Default for PopupResponse {
    fn default() -> Self {
        PopupResponse {
            answer: ::std::string::String::new(),
            success: false,
        }
    }
}

impl ros2_client::Message for PopupResponse {}


pub struct Popup;
impl ros2_client::Service for Popup {
    type Request = PopupRequest;
    type Response = PopupResponse;

    fn request_type_name(&self) -> &str { "PopupRequest" }
    fn response_type_name(&self) -> &str { "PopupResponse" }
}
