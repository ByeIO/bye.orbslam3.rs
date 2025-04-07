use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileWriteRequest {
    pub file_path: ::std::string::String,
    pub offset: u64,
    pub data: Vec<u8>,
}

impl Default for FileWriteRequest {
    fn default() -> Self {
        FileWriteRequest {
            file_path: ::std::string::String::new(),
            offset: 0,
            data: Vec::new(),
        }
    }
}

impl ros2_client::Message for FileWriteRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileWriteResponse {
    pub success: bool,
    pub r_errno: i32,
}

impl Default for FileWriteResponse {
    fn default() -> Self {
        FileWriteResponse {
            success: false,
            r_errno: 0,
        }
    }
}

impl ros2_client::Message for FileWriteResponse {}


pub struct FileWrite;
impl ros2_client::Service for FileWrite {
    type Request = FileWriteRequest;
    type Response = FileWriteResponse;

    fn request_type_name(&self) -> &str { "FileWriteRequest" }
    fn response_type_name(&self) -> &str { "FileWriteResponse" }
}
