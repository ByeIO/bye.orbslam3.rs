use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NavVELNED {
    pub i_tow: u32,
    pub vel_n: i32,
    pub vel_e: i32,
    pub vel_d: i32,
    pub speed: u32,
    pub g_speed: u32,
    pub heading: i32,
    pub s_acc: u32,
    pub c_acc: u32,
}

impl NavVELNED {
    pub const CLASS_ID: u8 = 1;
    pub const MESSAGE_ID: u8 = 18;
}

impl Default for NavVELNED {
    fn default() -> Self {
        NavVELNED {
            i_tow: 0,
            vel_n: 0,
            vel_e: 0,
            vel_d: 0,
            speed: 0,
            g_speed: 0,
            heading: 0,
            s_acc: 0,
            c_acc: 0,
        }
    }
}

impl ros2_client::Message for NavVELNED {}
