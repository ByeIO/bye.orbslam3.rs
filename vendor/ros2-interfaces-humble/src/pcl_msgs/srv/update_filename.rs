use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateFilenameRequest {
    pub filename: ::std::string::String,
}

impl Default for UpdateFilenameRequest {
    fn default() -> Self {
        UpdateFilenameRequest {
            filename: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for UpdateFilenameRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateFilenameResponse {
    pub success: bool,
}

impl Default for UpdateFilenameResponse {
    fn default() -> Self {
        UpdateFilenameResponse {
            success: false,
        }
    }
}

impl ros2_client::Message for UpdateFilenameResponse {}


pub struct UpdateFilename;
impl ros2_client::Service for UpdateFilename {
    type Request = UpdateFilenameRequest;
    type Response = UpdateFilenameResponse;

    fn request_type_name(&self) -> &str { "UpdateFilenameRequest" }
    fn response_type_name(&self) -> &str { "UpdateFilenameResponse" }
}
