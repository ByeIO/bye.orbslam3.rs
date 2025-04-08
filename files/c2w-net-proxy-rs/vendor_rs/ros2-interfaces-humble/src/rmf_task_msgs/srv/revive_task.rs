use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReviveTaskRequest {
    pub requester: ::std::string::String,
    pub task_id: ::std::string::String,
}

impl Default for ReviveTaskRequest {
    fn default() -> Self {
        ReviveTaskRequest {
            requester: ::std::string::String::new(),
            task_id: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for ReviveTaskRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReviveTaskResponse {
    pub success: bool,
}

impl Default for ReviveTaskResponse {
    fn default() -> Self {
        ReviveTaskResponse {
            success: false,
        }
    }
}

impl ros2_client::Message for ReviveTaskResponse {}


pub struct ReviveTask;
impl ros2_client::Service for ReviveTask {
    type Request = ReviveTaskRequest;
    type Response = ReviveTaskResponse;

    fn request_type_name(&self) -> &str { "ReviveTaskRequest" }
    fn response_type_name(&self) -> &str { "ReviveTaskResponse" }
}
