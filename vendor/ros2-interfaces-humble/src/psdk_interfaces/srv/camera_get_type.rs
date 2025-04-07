use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CameraGetTypeRequest {
    pub payload_index: u8, // default: 1
}

impl Default for CameraGetTypeRequest {
    fn default() -> Self {
        CameraGetTypeRequest {
            payload_index: 1,
        }
    }
}

impl ros2_client::Message for CameraGetTypeRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CameraGetTypeResponse {
    pub success: bool,
    pub camera_type: ::std::string::String,
}

impl Default for CameraGetTypeResponse {
    fn default() -> Self {
        CameraGetTypeResponse {
            success: false,
            camera_type: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for CameraGetTypeResponse {}


pub struct CameraGetType;
impl ros2_client::Service for CameraGetType {
    type Request = CameraGetTypeRequest;
    type Response = CameraGetTypeResponse;

    fn request_type_name(&self) -> &str { "CameraGetTypeRequest" }
    fn response_type_name(&self) -> &str { "CameraGetTypeResponse" }
}
