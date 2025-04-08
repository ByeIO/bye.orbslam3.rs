use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RemoveMotionRequest {
    pub motion_key: ::std::string::String,
}

impl Default for RemoveMotionRequest {
    fn default() -> Self {
        RemoveMotionRequest {
            motion_key: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for RemoveMotionRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RemoveMotionResponse {
    pub success: bool,
}

impl Default for RemoveMotionResponse {
    fn default() -> Self {
        RemoveMotionResponse {
            success: false,
        }
    }
}

impl ros2_client::Message for RemoveMotionResponse {}


pub struct RemoveMotion;
impl ros2_client::Service for RemoveMotion {
    type Request = RemoveMotionRequest;
    type Response = RemoveMotionResponse;

    fn request_type_name(&self) -> &str { "RemoveMotionRequest" }
    fn response_type_name(&self) -> &str { "RemoveMotionResponse" }
}
