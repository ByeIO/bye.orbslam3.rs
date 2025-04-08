use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CONmtIDRequest {
    pub nmtcommand: u8,
    pub nodeid: u8,
}

impl Default for CONmtIDRequest {
    fn default() -> Self {
        CONmtIDRequest {
            nmtcommand: 0,
            nodeid: 0,
        }
    }
}

impl ros2_client::Message for CONmtIDRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CONmtIDResponse {
    pub success: bool,
}

impl Default for CONmtIDResponse {
    fn default() -> Self {
        CONmtIDResponse {
            success: false,
        }
    }
}

impl ros2_client::Message for CONmtIDResponse {}


pub struct CONmtID;
impl ros2_client::Service for CONmtID {
    type Request = CONmtIDRequest;
    type Response = CONmtIDResponse;

    fn request_type_name(&self) -> &str { "CONmtIDRequest" }
    fn response_type_name(&self) -> &str { "CONmtIDResponse" }
}
