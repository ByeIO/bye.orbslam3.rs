use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddToLogRequest {
    pub message: ::std::string::String,
}

impl Default for AddToLogRequest {
    fn default() -> Self {
        AddToLogRequest {
            message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for AddToLogRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddToLogResponse {
    pub answer: ::std::string::String,
    pub success: bool,
}

impl Default for AddToLogResponse {
    fn default() -> Self {
        AddToLogResponse {
            answer: ::std::string::String::new(),
            success: false,
        }
    }
}

impl ros2_client::Message for AddToLogResponse {}


pub struct AddToLog;
impl ros2_client::Service for AddToLog {
    type Request = AddToLogRequest;
    type Response = AddToLogResponse;

    fn request_type_name(&self) -> &str { "AddToLogRequest" }
    fn response_type_name(&self) -> &str { "AddToLogResponse" }
}
