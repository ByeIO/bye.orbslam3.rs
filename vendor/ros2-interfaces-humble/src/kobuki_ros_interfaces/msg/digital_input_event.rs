use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DigitalInputEvent {
    pub values: [bool; 4],
}

impl Default for DigitalInputEvent {
    fn default() -> Self {
        DigitalInputEvent {
            values: [false; 4],
        }
    }
}

impl ros2_client::Message for DigitalInputEvent {}
