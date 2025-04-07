use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct COWriteRequest {
    pub index: u16,
    pub subindex: u8,
    pub data: u32,
}

impl Default for COWriteRequest {
    fn default() -> Self {
        COWriteRequest {
            index: 0,
            subindex: 0,
            data: 0,
        }
    }
}

impl ros2_client::Message for COWriteRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct COWriteResponse {
    pub success: bool,
}

impl Default for COWriteResponse {
    fn default() -> Self {
        COWriteResponse {
            success: false,
        }
    }
}

impl ros2_client::Message for COWriteResponse {}


pub struct COWrite;
impl ros2_client::Service for COWrite {
    type Request = COWriteRequest;
    type Response = COWriteResponse;

    fn request_type_name(&self) -> &str { "COWriteRequest" }
    fn response_type_name(&self) -> &str { "COWriteResponse" }
}
