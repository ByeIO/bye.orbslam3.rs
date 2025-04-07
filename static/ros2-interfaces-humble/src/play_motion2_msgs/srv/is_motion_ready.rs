use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IsMotionReadyRequest {
    pub motion_key: ::std::string::String,
}

impl Default for IsMotionReadyRequest {
    fn default() -> Self {
        IsMotionReadyRequest {
            motion_key: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for IsMotionReadyRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IsMotionReadyResponse {
    pub is_ready: bool,
}

impl Default for IsMotionReadyResponse {
    fn default() -> Self {
        IsMotionReadyResponse {
            is_ready: false,
        }
    }
}

impl ros2_client::Message for IsMotionReadyResponse {}


pub struct IsMotionReady;
impl ros2_client::Service for IsMotionReady {
    type Request = IsMotionReadyRequest;
    type Response = IsMotionReadyResponse;

    fn request_type_name(&self) -> &str { "IsMotionReadyRequest" }
    fn response_type_name(&self) -> &str { "IsMotionReadyResponse" }
}
