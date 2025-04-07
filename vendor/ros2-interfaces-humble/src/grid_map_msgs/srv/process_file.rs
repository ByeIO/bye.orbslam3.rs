use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProcessFileRequest {
    pub file_path: ::std::string::String,
    pub topic_name: ::std::string::String,
}

impl Default for ProcessFileRequest {
    fn default() -> Self {
        ProcessFileRequest {
            file_path: ::std::string::String::new(),
            topic_name: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for ProcessFileRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProcessFileResponse {
    pub success: bool,
}

impl Default for ProcessFileResponse {
    fn default() -> Self {
        ProcessFileResponse {
            success: false,
        }
    }
}

impl ros2_client::Message for ProcessFileResponse {}


pub struct ProcessFile;
impl ros2_client::Service for ProcessFile {
    type Request = ProcessFileRequest;
    type Response = ProcessFileResponse;

    fn request_type_name(&self) -> &str { "ProcessFileRequest" }
    fn response_type_name(&self) -> &str { "ProcessFileResponse" }
}
