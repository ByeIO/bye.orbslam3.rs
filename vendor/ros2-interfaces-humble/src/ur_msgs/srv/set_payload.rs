use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetPayloadRequest {
    pub mass: f32,
    pub center_of_gravity: crate::geometry_msgs::msg::Vector3,
}

impl Default for SetPayloadRequest {
    fn default() -> Self {
        SetPayloadRequest {
            mass: 0.0,
            center_of_gravity: crate::geometry_msgs::msg::Vector3::default(),
        }
    }
}

impl ros2_client::Message for SetPayloadRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetPayloadResponse {
    pub success: bool,
}

impl Default for SetPayloadResponse {
    fn default() -> Self {
        SetPayloadResponse {
            success: false,
        }
    }
}

impl ros2_client::Message for SetPayloadResponse {}


pub struct SetPayload;
impl ros2_client::Service for SetPayload {
    type Request = SetPayloadRequest;
    type Response = SetPayloadResponse;

    fn request_type_name(&self) -> &str { "SetPayloadRequest" }
    fn response_type_name(&self) -> &str { "SetPayloadResponse" }
}
