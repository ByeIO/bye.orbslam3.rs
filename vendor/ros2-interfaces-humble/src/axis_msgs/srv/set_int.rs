use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetIntRequest {
    pub data: i32,
}

impl Default for SetIntRequest {
    fn default() -> Self {
        SetIntRequest {
            data: 0,
        }
    }
}

impl ros2_client::Message for SetIntRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetIntResponse {
    pub success: bool,
    pub message: ::std::string::String,
}

impl Default for SetIntResponse {
    fn default() -> Self {
        SetIntResponse {
            success: false,
            message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for SetIntResponse {}


pub struct SetInt;
impl ros2_client::Service for SetInt {
    type Request = SetIntRequest;
    type Response = SetIntResponse;

    fn request_type_name(&self) -> &str { "SetIntRequest" }
    fn response_type_name(&self) -> &str { "SetIntResponse" }
}
