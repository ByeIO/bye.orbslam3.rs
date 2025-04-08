use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommandTriggerControlRequest {
    pub trigger_enable: bool,
    pub sequence_reset: bool,
    pub trigger_pause: bool,
}

impl Default for CommandTriggerControlRequest {
    fn default() -> Self {
        CommandTriggerControlRequest {
            trigger_enable: false,
            sequence_reset: false,
            trigger_pause: false,
        }
    }
}

impl ros2_client::Message for CommandTriggerControlRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommandTriggerControlResponse {
    pub success: bool,
    pub result: u8,
}

impl Default for CommandTriggerControlResponse {
    fn default() -> Self {
        CommandTriggerControlResponse {
            success: false,
            result: 0,
        }
    }
}

impl ros2_client::Message for CommandTriggerControlResponse {}


pub struct CommandTriggerControl;
impl ros2_client::Service for CommandTriggerControl {
    type Request = CommandTriggerControlRequest;
    type Response = CommandTriggerControlResponse;

    fn request_type_name(&self) -> &str { "CommandTriggerControlRequest" }
    fn response_type_name(&self) -> &str { "CommandTriggerControlResponse" }
}
