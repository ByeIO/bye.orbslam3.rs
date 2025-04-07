use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AdaptiveCruiseControlCommand {
    pub header: crate::std_msgs::msg::Header,
    pub msg_counter: u8,
    pub set_speed: f32,
    pub set: u16,
    pub resume: u16,
    pub cancel: u16,
    pub speed_up: u16,
    pub slow_down: u16,
    pub further: u16,
    pub closer: u16,
}

impl Default for AdaptiveCruiseControlCommand {
    fn default() -> Self {
        AdaptiveCruiseControlCommand {
            header: crate::std_msgs::msg::Header::default(),
            msg_counter: 0,
            set_speed: 0.0,
            set: 0,
            resume: 0,
            cancel: 0,
            speed_up: 0,
            slow_down: 0,
            further: 0,
            closer: 0,
        }
    }
}

impl ros2_client::Message for AdaptiveCruiseControlCommand {}
