use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetPlatformStateMachineEventRequest {
    pub event: crate::as2_msgs::msg::PlatformStateMachineEvent,
}

impl Default for SetPlatformStateMachineEventRequest {
    fn default() -> Self {
        SetPlatformStateMachineEventRequest {
            event: crate::as2_msgs::msg::PlatformStateMachineEvent::default(),
        }
    }
}

impl ros2_client::Message for SetPlatformStateMachineEventRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetPlatformStateMachineEventResponse {
    pub success: bool,
    pub current_state: crate::as2_msgs::msg::PlatformStatus,
}

impl Default for SetPlatformStateMachineEventResponse {
    fn default() -> Self {
        SetPlatformStateMachineEventResponse {
            success: false,
            current_state: crate::as2_msgs::msg::PlatformStatus::default(),
        }
    }
}

impl ros2_client::Message for SetPlatformStateMachineEventResponse {}


pub struct SetPlatformStateMachineEvent;
impl ros2_client::Service for SetPlatformStateMachineEvent {
    type Request = SetPlatformStateMachineEventRequest;
    type Response = SetPlatformStateMachineEventResponse;

    fn request_type_name(&self) -> &str { "SetPlatformStateMachineEventRequest" }
    fn response_type_name(&self) -> &str { "SetPlatformStateMachineEventResponse" }
}
