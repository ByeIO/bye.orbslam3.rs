use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileCloseRequest {
    pub file_path: ::std::string::String,
}

impl Default for FileCloseRequest {
    fn default() -> Self {
        FileCloseRequest {
            file_path: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for FileCloseRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileCloseResponse {
    pub success: bool,
    pub r_errno: i32,
}

impl Default for FileCloseResponse {
    fn default() -> Self {
        FileCloseResponse {
            success: false,
            r_errno: 0,
        }
    }
}

impl ros2_client::Message for FileCloseResponse {}


pub struct FileClose;
impl ros2_client::Service for FileClose {
    type Request = FileCloseRequest;
    type Response = FileCloseResponse;

    fn request_type_name(&self) -> &str { "FileCloseRequest" }
    fn response_type_name(&self) -> &str { "FileCloseResponse" }
}
