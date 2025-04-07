use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StopRecordingRequest {
    pub discard: bool,
    pub filename: ::std::string::String,
}

impl Default for StopRecordingRequest {
    fn default() -> Self {
        StopRecordingRequest {
            discard: false,
            filename: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for StopRecordingRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StopRecordingResponse {
    pub path: ::std::string::String,
    pub success: bool,
}

impl Default for StopRecordingResponse {
    fn default() -> Self {
        StopRecordingResponse {
            path: ::std::string::String::new(),
            success: false,
        }
    }
}

impl ros2_client::Message for StopRecordingResponse {}


pub struct StopRecording;
impl ros2_client::Service for StopRecording {
    type Request = StopRecordingRequest;
    type Response = StopRecordingResponse;

    fn request_type_name(&self) -> &str { "StopRecordingRequest" }
    fn response_type_name(&self) -> &str { "StopRecordingResponse" }
}
