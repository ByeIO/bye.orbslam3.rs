use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileListRequest {
    pub dir_path: ::std::string::String,
}

impl Default for FileListRequest {
    fn default() -> Self {
        FileListRequest {
            dir_path: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for FileListRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileListResponse {
    pub list: Vec<crate::mavros_msgs::msg::FileEntry>,
    pub success: bool,
    pub r_errno: i32,
}

impl Default for FileListResponse {
    fn default() -> Self {
        FileListResponse {
            list: Vec::new(),
            success: false,
            r_errno: 0,
        }
    }
}

impl ros2_client::Message for FileListResponse {}


pub struct FileList;
impl ros2_client::Service for FileList {
    type Request = FileListRequest;
    type Response = FileListResponse;

    fn request_type_name(&self) -> &str { "FileListRequest" }
    fn response_type_name(&self) -> &str { "FileListResponse" }
}
