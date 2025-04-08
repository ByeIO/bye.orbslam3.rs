use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CONodeRequest {
    pub nodeid: u8,
}

impl Default for CONodeRequest {
    fn default() -> Self {
        CONodeRequest {
            nodeid: 0,
        }
    }
}

impl ros2_client::Message for CONodeRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CONodeResponse {
    pub success: bool,
}

impl Default for CONodeResponse {
    fn default() -> Self {
        CONodeResponse {
            success: false,
        }
    }
}

impl ros2_client::Message for CONodeResponse {}


pub struct CONode;
impl ros2_client::Service for CONode {
    type Request = CONodeRequest;
    type Response = CONodeResponse;

    fn request_type_name(&self) -> &str { "CONodeRequest" }
    fn response_type_name(&self) -> &str { "CONodeResponse" }
}
