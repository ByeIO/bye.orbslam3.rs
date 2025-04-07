use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetHomeFromGPSRequest {
    pub latitude: f64,
    pub longitude: f64,
}

impl Default for SetHomeFromGPSRequest {
    fn default() -> Self {
        SetHomeFromGPSRequest {
            latitude: 0.0,
            longitude: 0.0,
        }
    }
}

impl ros2_client::Message for SetHomeFromGPSRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetHomeFromGPSResponse {
    pub success: bool,
}

impl Default for SetHomeFromGPSResponse {
    fn default() -> Self {
        SetHomeFromGPSResponse {
            success: false,
        }
    }
}

impl ros2_client::Message for SetHomeFromGPSResponse {}


pub struct SetHomeFromGPS;
impl ros2_client::Service for SetHomeFromGPS {
    type Request = SetHomeFromGPSRequest;
    type Response = SetHomeFromGPSResponse;

    fn request_type_name(&self) -> &str { "SetHomeFromGPSRequest" }
    fn response_type_name(&self) -> &str { "SetHomeFromGPSResponse" }
}
