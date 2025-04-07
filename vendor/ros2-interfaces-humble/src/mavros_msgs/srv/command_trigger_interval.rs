use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommandTriggerIntervalRequest {
    pub cycle_time: f32,
    pub integration_time: f32,
}

impl Default for CommandTriggerIntervalRequest {
    fn default() -> Self {
        CommandTriggerIntervalRequest {
            cycle_time: 0.0,
            integration_time: 0.0,
        }
    }
}

impl ros2_client::Message for CommandTriggerIntervalRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommandTriggerIntervalResponse {
    pub success: bool,
    pub result: u8,
}

impl Default for CommandTriggerIntervalResponse {
    fn default() -> Self {
        CommandTriggerIntervalResponse {
            success: false,
            result: 0,
        }
    }
}

impl ros2_client::Message for CommandTriggerIntervalResponse {}


pub struct CommandTriggerInterval;
impl ros2_client::Service for CommandTriggerInterval {
    type Request = CommandTriggerIntervalRequest;
    type Response = CommandTriggerIntervalResponse;

    fn request_type_name(&self) -> &str { "CommandTriggerIntervalRequest" }
    fn response_type_name(&self) -> &str { "CommandTriggerIntervalResponse" }
}
