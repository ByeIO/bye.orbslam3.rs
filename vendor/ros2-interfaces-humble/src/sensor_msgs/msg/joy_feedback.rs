use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JoyFeedback {
    #[serde(rename = "type")]    pub type_: u8,
    pub id: u8,
    pub intensity: f32,
}

impl JoyFeedback {
    pub const TYPE_LED: u8 = 0;
    pub const TYPE_RUMBLE: u8 = 1;
    pub const TYPE_BUZZER: u8 = 2;
}

impl Default for JoyFeedback {
    fn default() -> Self {
        JoyFeedback {
            type_: 0,
            id: 0,
            intensity: 0.0,
        }
    }
}

impl ros2_client::Message for JoyFeedback {}
