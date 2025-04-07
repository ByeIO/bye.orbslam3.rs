use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoadGeometryFromFileRequest {
    pub file_path_and_name: ::std::string::String,
}

impl Default for LoadGeometryFromFileRequest {
    fn default() -> Self {
        LoadGeometryFromFileRequest {
            file_path_and_name: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for LoadGeometryFromFileRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoadGeometryFromFileResponse {
    pub success: bool,
}

impl Default for LoadGeometryFromFileResponse {
    fn default() -> Self {
        LoadGeometryFromFileResponse {
            success: false,
        }
    }
}

impl ros2_client::Message for LoadGeometryFromFileResponse {}


pub struct LoadGeometryFromFile;
impl ros2_client::Service for LoadGeometryFromFile {
    type Request = LoadGeometryFromFileRequest;
    type Response = LoadGeometryFromFileResponse;

    fn request_type_name(&self) -> &str { "LoadGeometryFromFileRequest" }
    fn response_type_name(&self) -> &str { "LoadGeometryFromFileResponse" }
}
