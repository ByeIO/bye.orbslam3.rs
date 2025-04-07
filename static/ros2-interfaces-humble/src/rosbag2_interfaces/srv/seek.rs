use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SeekRequest {
    pub time: crate::builtin_interfaces::msg::Time,
}

impl Default for SeekRequest {
    fn default() -> Self {
        SeekRequest {
            time: crate::builtin_interfaces::msg::Time::default(),
        }
    }
}

impl ros2_client::Message for SeekRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SeekResponse {
    pub success: bool,
}

impl Default for SeekResponse {
    fn default() -> Self {
        SeekResponse {
            success: false,
        }
    }
}

impl ros2_client::Message for SeekResponse {}


pub struct Seek;
impl ros2_client::Service for Seek {
    type Request = SeekRequest;
    type Response = SeekResponse;

    fn request_type_name(&self) -> &str { "SeekRequest" }
    fn response_type_name(&self) -> &str { "SeekResponse" }
}
