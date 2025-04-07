use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubmitTaskRequest {
    pub requester: ::std::string::String,
    pub description: crate::rmf_task_msgs::msg::TaskDescription,
}

impl Default for SubmitTaskRequest {
    fn default() -> Self {
        SubmitTaskRequest {
            requester: ::std::string::String::new(),
            description: crate::rmf_task_msgs::msg::TaskDescription::default(),
        }
    }
}

impl ros2_client::Message for SubmitTaskRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubmitTaskResponse {
    pub success: bool,
    pub task_id: ::std::string::String,
    pub message: ::std::string::String,
}

impl Default for SubmitTaskResponse {
    fn default() -> Self {
        SubmitTaskResponse {
            success: false,
            task_id: ::std::string::String::new(),
            message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for SubmitTaskResponse {}


pub struct SubmitTask;
impl ros2_client::Service for SubmitTask {
    type Request = SubmitTaskRequest;
    type Response = SubmitTaskResponse;

    fn request_type_name(&self) -> &str { "SubmitTaskRequest" }
    fn response_type_name(&self) -> &str { "SubmitTaskResponse" }
}
