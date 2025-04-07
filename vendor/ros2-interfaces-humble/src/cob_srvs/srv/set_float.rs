use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetFloatRequest {
    pub data: f32,
}

impl Default for SetFloatRequest {
    fn default() -> Self {
        SetFloatRequest {
            data: 0.0,
        }
    }
}

impl ros2_client::Message for SetFloatRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetFloatResponse {
    pub success: bool,
    pub message: ::std::string::String,
}

impl Default for SetFloatResponse {
    fn default() -> Self {
        SetFloatResponse {
            success: false,
            message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for SetFloatResponse {}


pub struct SetFloat;
impl ros2_client::Service for SetFloat {
    type Request = SetFloatRequest;
    type Response = SetFloatResponse;

    fn request_type_name(&self) -> &str { "SetFloatRequest" }
    fn response_type_name(&self) -> &str { "SetFloatResponse" }
}
