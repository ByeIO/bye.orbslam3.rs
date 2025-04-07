use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SaveGeometryToFileRequest {
    pub file_path_and_name: ::std::string::String,
}

impl Default for SaveGeometryToFileRequest {
    fn default() -> Self {
        SaveGeometryToFileRequest {
            file_path_and_name: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for SaveGeometryToFileRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SaveGeometryToFileResponse {
    pub success: bool,
}

impl Default for SaveGeometryToFileResponse {
    fn default() -> Self {
        SaveGeometryToFileResponse {
            success: false,
        }
    }
}

impl ros2_client::Message for SaveGeometryToFileResponse {}


pub struct SaveGeometryToFile;
impl ros2_client::Service for SaveGeometryToFile {
    type Request = SaveGeometryToFileRequest;
    type Response = SaveGeometryToFileResponse;

    fn request_type_name(&self) -> &str { "SaveGeometryToFileRequest" }
    fn response_type_name(&self) -> &str { "SaveGeometryToFileResponse" }
}
