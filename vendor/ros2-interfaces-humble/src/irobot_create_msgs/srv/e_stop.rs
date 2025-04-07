use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EStopRequest {
    pub e_stop_on: bool,
}

impl Default for EStopRequest {
    fn default() -> Self {
        EStopRequest {
            e_stop_on: false,
        }
    }
}

impl ros2_client::Message for EStopRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EStopResponse {
    pub success: bool,
    pub message: ::std::string::String,
}

impl Default for EStopResponse {
    fn default() -> Self {
        EStopResponse {
            success: false,
            message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for EStopResponse {}


pub struct EStop;
impl ros2_client::Service for EStop {
    type Request = EStopRequest;
    type Response = EStopResponse;

    fn request_type_name(&self) -> &str { "EStopRequest" }
    fn response_type_name(&self) -> &str { "EStopResponse" }
}
