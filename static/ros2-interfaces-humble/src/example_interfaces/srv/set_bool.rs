use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetBoolRequest {
    pub data: bool,
}

impl Default for SetBoolRequest {
    fn default() -> Self {
        SetBoolRequest {
            data: false,
        }
    }
}

impl ros2_client::Message for SetBoolRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetBoolResponse {
    pub success: bool,
    pub message: ::std::string::String,
}

impl Default for SetBoolResponse {
    fn default() -> Self {
        SetBoolResponse {
            success: false,
            message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for SetBoolResponse {}


pub struct SetBool;
impl ros2_client::Service for SetBool {
    type Request = SetBoolRequest;
    type Response = SetBoolResponse;

    fn request_type_name(&self) -> &str { "SetBoolRequest" }
    fn response_type_name(&self) -> &str { "SetBoolResponse" }
}
