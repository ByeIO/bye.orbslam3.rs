use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetControlModeRequest {
    pub control_mode: crate::as2_msgs::msg::ControlMode,
}

impl Default for SetControlModeRequest {
    fn default() -> Self {
        SetControlModeRequest {
            control_mode: crate::as2_msgs::msg::ControlMode::default(),
        }
    }
}

impl ros2_client::Message for SetControlModeRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetControlModeResponse {
    pub success: bool,
}

impl Default for SetControlModeResponse {
    fn default() -> Self {
        SetControlModeResponse {
            success: false,
        }
    }
}

impl ros2_client::Message for SetControlModeResponse {}


pub struct SetControlMode;
impl ros2_client::Service for SetControlMode {
    type Request = SetControlModeRequest;
    type Response = SetControlModeResponse;

    fn request_type_name(&self) -> &str { "SetControlModeRequest" }
    fn response_type_name(&self) -> &str { "SetControlModeResponse" }
}
