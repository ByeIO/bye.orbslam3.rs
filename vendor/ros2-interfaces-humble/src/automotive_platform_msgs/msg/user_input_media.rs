use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserInputMedia {
    pub header: crate::std_msgs::msg::Header,
    pub btn_vol_up: bool,
    pub btn_vol_down: bool,
    pub btn_mute: bool,
    pub btn_next: bool,
    pub btn_prev: bool,
    pub btn_next_hang_up: bool,
    pub btn_prev_answer: bool,
    pub btn_hang_up: bool,
    pub btn_answer: bool,
    pub btn_play: bool,
    pub btn_pause: bool,
    pub btn_play_pause: bool,
    pub btn_mode: bool,
}

impl Default for UserInputMedia {
    fn default() -> Self {
        UserInputMedia {
            header: crate::std_msgs::msg::Header::default(),
            btn_vol_up: false,
            btn_vol_down: false,
            btn_mute: false,
            btn_next: false,
            btn_prev: false,
            btn_next_hang_up: false,
            btn_prev_answer: false,
            btn_hang_up: false,
            btn_answer: false,
            btn_play: false,
            btn_pause: false,
            btn_play_pause: false,
            btn_mode: false,
        }
    }
}

impl ros2_client::Message for UserInputMedia {}
