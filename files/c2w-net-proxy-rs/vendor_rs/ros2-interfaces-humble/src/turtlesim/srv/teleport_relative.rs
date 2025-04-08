use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TeleportRelativeRequest {
    pub linear: f32,
    pub angular: f32,
}

impl Default for TeleportRelativeRequest {
    fn default() -> Self {
        TeleportRelativeRequest {
            linear: 0.0,
            angular: 0.0,
        }
    }
}

impl ros2_client::Message for TeleportRelativeRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TeleportRelativeResponse {

}

impl Default for TeleportRelativeResponse {
    fn default() -> Self {
        TeleportRelativeResponse {

        }
    }
}

impl ros2_client::Message for TeleportRelativeResponse {}


pub struct TeleportRelative;
impl ros2_client::Service for TeleportRelative {
    type Request = TeleportRelativeRequest;
    type Response = TeleportRelativeResponse;

    fn request_type_name(&self) -> &str { "TeleportRelativeRequest" }
    fn response_type_name(&self) -> &str { "TeleportRelativeResponse" }
}
