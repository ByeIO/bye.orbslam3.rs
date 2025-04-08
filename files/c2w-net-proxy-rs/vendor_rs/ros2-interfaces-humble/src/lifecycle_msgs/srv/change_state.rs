use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChangeStateRequest {
    pub transition: crate::lifecycle_msgs::msg::Transition,
}

impl Default for ChangeStateRequest {
    fn default() -> Self {
        ChangeStateRequest {
            transition: crate::lifecycle_msgs::msg::Transition::default(),
        }
    }
}

impl ros2_client::Message for ChangeStateRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChangeStateResponse {
    pub success: bool,
}

impl Default for ChangeStateResponse {
    fn default() -> Self {
        ChangeStateResponse {
            success: false,
        }
    }
}

impl ros2_client::Message for ChangeStateResponse {}


pub struct ChangeState;
impl ros2_client::Service for ChangeState {
    type Request = ChangeStateRequest;
    type Response = ChangeStateResponse;

    fn request_type_name(&self) -> &str { "ChangeStateRequest" }
    fn response_type_name(&self) -> &str { "ChangeStateResponse" }
}
