use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CancelAllRequest {
    pub group: ::std::string::String,
}

impl Default for CancelAllRequest {
    fn default() -> Self {
        CancelAllRequest {
            group: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for CancelAllRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CancelAllResponse {
    pub success: bool,
    pub message: ::std::string::String,
}

impl Default for CancelAllResponse {
    fn default() -> Self {
        CancelAllResponse {
            success: false,
            message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for CancelAllResponse {}


pub struct CancelAll;
impl ros2_client::Service for CancelAll {
    type Request = CancelAllRequest;
    type Response = CancelAllResponse;

    fn request_type_name(&self) -> &str { "CancelAllRequest" }
    fn response_type_name(&self) -> &str { "CancelAllResponse" }
}
