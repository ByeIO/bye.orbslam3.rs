use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessageIntervalRequest {
    pub message_id: u32,
    pub message_rate: f32,
}

impl Default for MessageIntervalRequest {
    fn default() -> Self {
        MessageIntervalRequest {
            message_id: 0,
            message_rate: 0.0,
        }
    }
}

impl ros2_client::Message for MessageIntervalRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessageIntervalResponse {
    pub success: bool,
}

impl Default for MessageIntervalResponse {
    fn default() -> Self {
        MessageIntervalResponse {
            success: false,
        }
    }
}

impl ros2_client::Message for MessageIntervalResponse {}


pub struct MessageInterval;
impl ros2_client::Service for MessageInterval {
    type Request = MessageIntervalRequest;
    type Response = MessageIntervalResponse;

    fn request_type_name(&self) -> &str { "MessageIntervalRequest" }
    fn response_type_name(&self) -> &str { "MessageIntervalResponse" }
}
