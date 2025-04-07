use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Bumper {
    pub header: crate::std_msgs::msg::Header,
    pub is_left_pressed: bool,
    pub is_right_pressed: bool,
    pub is_light_left: bool,
    pub is_light_front_left: bool,
    pub is_light_center_left: bool,
    pub is_light_center_right: bool,
    pub is_light_front_right: bool,
    pub is_light_right: bool,
    pub light_signal_left: u16,
    pub light_signal_front_left: u16,
    pub light_signal_center_left: u16,
    pub light_signal_center_right: u16,
    pub light_signal_front_right: u16,
    pub light_signal_right: u16,
}

impl Default for Bumper {
    fn default() -> Self {
        Bumper {
            header: crate::std_msgs::msg::Header::default(),
            is_left_pressed: false,
            is_right_pressed: false,
            is_light_left: false,
            is_light_front_left: false,
            is_light_center_left: false,
            is_light_center_right: false,
            is_light_front_right: false,
            is_light_right: false,
            light_signal_left: 0,
            light_signal_front_left: 0,
            light_signal_center_left: 0,
            light_signal_center_right: 0,
            light_signal_front_right: 0,
            light_signal_right: 0,
        }
    }
}

impl ros2_client::Message for Bumper {}
