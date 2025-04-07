use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetStringRequest {
    pub value: ::std::string::String,
}

impl Default for SetStringRequest {
    fn default() -> Self {
        SetStringRequest {
            value: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for SetStringRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetStringResponse {
    pub success: bool,
}

impl Default for SetStringResponse {
    fn default() -> Self {
        SetStringResponse {
            success: false,
        }
    }
}

impl ros2_client::Message for SetStringResponse {}


pub struct SetString;
impl ros2_client::Service for SetString {
    type Request = SetStringRequest;
    type Response = SetStringResponse;

    fn request_type_name(&self) -> &str { "SetStringRequest" }
    fn response_type_name(&self) -> &str { "SetStringResponse" }
}
