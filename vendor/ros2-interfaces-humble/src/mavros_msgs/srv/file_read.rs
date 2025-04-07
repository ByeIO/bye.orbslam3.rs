use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileReadRequest {
    pub file_path: ::std::string::String,
    pub offset: u64,
    pub size: u64,
}

impl Default for FileReadRequest {
    fn default() -> Self {
        FileReadRequest {
            file_path: ::std::string::String::new(),
            offset: 0,
            size: 0,
        }
    }
}

impl ros2_client::Message for FileReadRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileReadResponse {
    pub data: Vec<u8>,
    pub success: bool,
    pub r_errno: i32,
}

impl Default for FileReadResponse {
    fn default() -> Self {
        FileReadResponse {
            data: Vec::new(),
            success: false,
            r_errno: 0,
        }
    }
}

impl ros2_client::Message for FileReadResponse {}


pub struct FileRead;
impl ros2_client::Service for FileRead {
    type Request = FileReadRequest;
    type Response = FileReadResponse;

    fn request_type_name(&self) -> &str { "FileReadRequest" }
    fn response_type_name(&self) -> &str { "FileReadResponse" }
}
