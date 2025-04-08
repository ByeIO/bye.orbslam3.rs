use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddMotionRequest {
    pub motion: crate::play_motion2_msgs::msg::Motion,
    pub overwrite: bool,
}

impl Default for AddMotionRequest {
    fn default() -> Self {
        AddMotionRequest {
            motion: crate::play_motion2_msgs::msg::Motion::default(),
            overwrite: false,
        }
    }
}

impl ros2_client::Message for AddMotionRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddMotionResponse {
    pub success: bool,
}

impl Default for AddMotionResponse {
    fn default() -> Self {
        AddMotionResponse {
            success: false,
        }
    }
}

impl ros2_client::Message for AddMotionResponse {}


pub struct AddMotion;
impl ros2_client::Service for AddMotion {
    type Request = AddMotionRequest;
    type Response = AddMotionResponse;

    fn request_type_name(&self) -> &str { "AddMotionRequest" }
    fn response_type_name(&self) -> &str { "AddMotionResponse" }
}
