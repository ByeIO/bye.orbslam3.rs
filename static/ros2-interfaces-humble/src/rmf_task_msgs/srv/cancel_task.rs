use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CancelTaskRequest {
    pub requester: ::std::string::String,
    pub task_id: ::std::string::String,
}

impl Default for CancelTaskRequest {
    fn default() -> Self {
        CancelTaskRequest {
            requester: ::std::string::String::new(),
            task_id: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for CancelTaskRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CancelTaskResponse {
    pub success: bool,
    pub message: ::std::string::String,
}

impl Default for CancelTaskResponse {
    fn default() -> Self {
        CancelTaskResponse {
            success: false,
            message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for CancelTaskResponse {}


pub struct CancelTask;
impl ros2_client::Service for CancelTask {
    type Request = CancelTaskRequest;
    type Response = CancelTaskResponse;

    fn request_type_name(&self) -> &str { "CancelTaskRequest" }
    fn response_type_name(&self) -> &str { "CancelTaskResponse" }
}
