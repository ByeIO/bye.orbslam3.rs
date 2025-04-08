use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetHardwareComponentStateRequest {
    pub name: ::std::string::String,
    pub target_state: crate::lifecycle_msgs::msg::State,
}

impl Default for SetHardwareComponentStateRequest {
    fn default() -> Self {
        SetHardwareComponentStateRequest {
            name: ::std::string::String::new(),
            target_state: crate::lifecycle_msgs::msg::State::default(),
        }
    }
}

impl ros2_client::Message for SetHardwareComponentStateRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetHardwareComponentStateResponse {
    pub ok: bool,
    pub state: crate::lifecycle_msgs::msg::State,
}

impl Default for SetHardwareComponentStateResponse {
    fn default() -> Self {
        SetHardwareComponentStateResponse {
            ok: false,
            state: crate::lifecycle_msgs::msg::State::default(),
        }
    }
}

impl ros2_client::Message for SetHardwareComponentStateResponse {}


pub struct SetHardwareComponentState;
impl ros2_client::Service for SetHardwareComponentState {
    type Request = SetHardwareComponentStateRequest;
    type Response = SetHardwareComponentStateResponse;

    fn request_type_name(&self) -> &str { "SetHardwareComponentStateRequest" }
    fn response_type_name(&self) -> &str { "SetHardwareComponentStateResponse" }
}
