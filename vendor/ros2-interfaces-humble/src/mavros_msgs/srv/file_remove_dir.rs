use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileRemoveDirRequest {
    pub dir_path: ::std::string::String,
}

impl Default for FileRemoveDirRequest {
    fn default() -> Self {
        FileRemoveDirRequest {
            dir_path: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for FileRemoveDirRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileRemoveDirResponse {
    pub success: bool,
    pub r_errno: i32,
}

impl Default for FileRemoveDirResponse {
    fn default() -> Self {
        FileRemoveDirResponse {
            success: false,
            r_errno: 0,
        }
    }
}

impl ros2_client::Message for FileRemoveDirResponse {}


pub struct FileRemoveDir;
impl ros2_client::Service for FileRemoveDir {
    type Request = FileRemoveDirRequest;
    type Response = FileRemoveDirResponse;

    fn request_type_name(&self) -> &str { "FileRemoveDirRequest" }
    fn response_type_name(&self) -> &str { "FileRemoveDirResponse" }
}
