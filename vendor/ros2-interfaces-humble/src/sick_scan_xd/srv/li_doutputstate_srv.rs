use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LIDoutputstateSrvRequest {
    pub active: bool,
}

impl Default for LIDoutputstateSrvRequest {
    fn default() -> Self {
        LIDoutputstateSrvRequest {
            active: false,
        }
    }
}

impl ros2_client::Message for LIDoutputstateSrvRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LIDoutputstateSrvResponse {
    pub success: bool,
}

impl Default for LIDoutputstateSrvResponse {
    fn default() -> Self {
        LIDoutputstateSrvResponse {
            success: false,
        }
    }
}

impl ros2_client::Message for LIDoutputstateSrvResponse {}


pub struct LIDoutputstateSrv;
impl ros2_client::Service for LIDoutputstateSrv {
    type Request = LIDoutputstateSrvRequest;
    type Response = LIDoutputstateSrvResponse;

    fn request_type_name(&self) -> &str { "LIDoutputstateSrvRequest" }
    fn response_type_name(&self) -> &str { "LIDoutputstateSrvResponse" }
}
