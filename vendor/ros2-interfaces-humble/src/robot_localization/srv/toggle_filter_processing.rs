use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToggleFilterProcessingRequest {
    pub on: bool,
}

impl Default for ToggleFilterProcessingRequest {
    fn default() -> Self {
        ToggleFilterProcessingRequest {
            on: false,
        }
    }
}

impl ros2_client::Message for ToggleFilterProcessingRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToggleFilterProcessingResponse {
    pub status: bool,
}

impl Default for ToggleFilterProcessingResponse {
    fn default() -> Self {
        ToggleFilterProcessingResponse {
            status: false,
        }
    }
}

impl ros2_client::Message for ToggleFilterProcessingResponse {}


pub struct ToggleFilterProcessing;
impl ros2_client::Service for ToggleFilterProcessing {
    type Request = ToggleFilterProcessingRequest;
    type Response = ToggleFilterProcessingResponse;

    fn request_type_name(&self) -> &str { "ToggleFilterProcessingRequest" }
    fn response_type_name(&self) -> &str { "ToggleFilterProcessingResponse" }
}
