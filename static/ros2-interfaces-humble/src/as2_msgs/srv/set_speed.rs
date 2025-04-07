use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetSpeedRequest {
    pub speed: crate::as2_msgs::msg::Speed,
}

impl Default for SetSpeedRequest {
    fn default() -> Self {
        SetSpeedRequest {
            speed: crate::as2_msgs::msg::Speed::default(),
        }
    }
}

impl ros2_client::Message for SetSpeedRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetSpeedResponse {
    pub success: bool,
}

impl Default for SetSpeedResponse {
    fn default() -> Self {
        SetSpeedResponse {
            success: false,
        }
    }
}

impl ros2_client::Message for SetSpeedResponse {}


pub struct SetSpeed;
impl ros2_client::Service for SetSpeed {
    type Request = SetSpeedRequest;
    type Response = SetSpeedResponse;

    fn request_type_name(&self) -> &str { "SetSpeedRequest" }
    fn response_type_name(&self) -> &str { "SetSpeedResponse" }
}
