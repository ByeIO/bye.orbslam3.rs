use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ManualControl {
    pub header: crate::std_msgs::msg::Header,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub r: f32,
    pub buttons: u16,
    pub buttons2: u16,
    pub enabled_extensions: u8,
    pub s: f32,
    pub t: f32,
    pub aux1: f32,
    pub aux2: f32,
    pub aux3: f32,
    pub aux4: f32,
    pub aux5: f32,
    pub aux6: f32,
}

impl Default for ManualControl {
    fn default() -> Self {
        ManualControl {
            header: crate::std_msgs::msg::Header::default(),
            x: 0.0,
            y: 0.0,
            z: 0.0,
            r: 0.0,
            buttons: 0,
            buttons2: 0,
            enabled_extensions: 0,
            s: 0.0,
            t: 0.0,
            aux1: 0.0,
            aux2: 0.0,
            aux3: 0.0,
            aux4: 0.0,
            aux5: 0.0,
            aux6: 0.0,
        }
    }
}

impl ros2_client::Message for ManualControl {}
