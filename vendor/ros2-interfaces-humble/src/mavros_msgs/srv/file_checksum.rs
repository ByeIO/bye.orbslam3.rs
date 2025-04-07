use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileChecksumRequest {
    pub file_path: ::std::string::String,
}

impl Default for FileChecksumRequest {
    fn default() -> Self {
        FileChecksumRequest {
            file_path: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for FileChecksumRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileChecksumResponse {
    pub crc32: u32,
    pub success: bool,
    pub r_errno: i32,
}

impl Default for FileChecksumResponse {
    fn default() -> Self {
        FileChecksumResponse {
            crc32: 0,
            success: false,
            r_errno: 0,
        }
    }
}

impl ros2_client::Message for FileChecksumResponse {}


pub struct FileChecksum;
impl ros2_client::Service for FileChecksum {
    type Request = FileChecksumRequest;
    type Response = FileChecksumResponse;

    fn request_type_name(&self) -> &str { "FileChecksumRequest" }
    fn response_type_name(&self) -> &str { "FileChecksumResponse" }
}
