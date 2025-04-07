use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileMakeDirRequest {
    pub dir_path: ::std::string::String,
}

impl Default for FileMakeDirRequest {
    fn default() -> Self {
        FileMakeDirRequest {
            dir_path: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for FileMakeDirRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileMakeDirResponse {
    pub success: bool,
    pub r_errno: i32,
}

impl Default for FileMakeDirResponse {
    fn default() -> Self {
        FileMakeDirResponse {
            success: false,
            r_errno: 0,
        }
    }
}

impl ros2_client::Message for FileMakeDirResponse {}


pub struct FileMakeDir;
impl ros2_client::Service for FileMakeDir {
    type Request = FileMakeDirRequest;
    type Response = FileMakeDirResponse;

    fn request_type_name(&self) -> &str { "FileMakeDirRequest" }
    fn response_type_name(&self) -> &str { "FileMakeDirResponse" }
}
