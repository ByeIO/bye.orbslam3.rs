use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct COHeartbeatIDRequest {
    pub nodeid: u8,
    pub heartbeat: u16,
}

impl Default for COHeartbeatIDRequest {
    fn default() -> Self {
        COHeartbeatIDRequest {
            nodeid: 0,
            heartbeat: 0,
        }
    }
}

impl ros2_client::Message for COHeartbeatIDRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct COHeartbeatIDResponse {
    pub success: bool,
}

impl Default for COHeartbeatIDResponse {
    fn default() -> Self {
        COHeartbeatIDResponse {
            success: false,
        }
    }
}

impl ros2_client::Message for COHeartbeatIDResponse {}


pub struct COHeartbeatID;
impl ros2_client::Service for COHeartbeatID {
    type Request = COHeartbeatIDRequest;
    type Response = COHeartbeatIDResponse;

    fn request_type_name(&self) -> &str { "COHeartbeatIDRequest" }
    fn response_type_name(&self) -> &str { "COHeartbeatIDResponse" }
}
