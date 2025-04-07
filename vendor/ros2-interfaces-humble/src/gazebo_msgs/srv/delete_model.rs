use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteModelRequest {
    pub model_name: ::std::string::String,
}

impl Default for DeleteModelRequest {
    fn default() -> Self {
        DeleteModelRequest {
            model_name: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for DeleteModelRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteModelResponse {
    pub success: bool,
    pub status_message: ::std::string::String,
}

impl Default for DeleteModelResponse {
    fn default() -> Self {
        DeleteModelResponse {
            success: false,
            status_message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for DeleteModelResponse {}


pub struct DeleteModel;
impl ros2_client::Service for DeleteModel {
    type Request = DeleteModelRequest;
    type Response = DeleteModelResponse;

    fn request_type_name(&self) -> &str { "DeleteModelRequest" }
    fn response_type_name(&self) -> &str { "DeleteModelResponse" }
}
