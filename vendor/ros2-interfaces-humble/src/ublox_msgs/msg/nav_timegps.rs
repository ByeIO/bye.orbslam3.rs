use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NavTIMEGPS {
    pub i_tow: u32,
    pub f_tow: i32,
    pub week: i16,
    pub leap_s: i8,
    pub valid: u8,
    pub t_acc: u32,
}

impl NavTIMEGPS {
    pub const CLASS_ID: u8 = 1;
    pub const MESSAGE_ID: u8 = 32;
    pub const VALID_TOW: u8 = 1;
    pub const VALID_WEEK: u8 = 2;
    pub const VALID_LEAP_S: u8 = 4;
}

impl Default for NavTIMEGPS {
    fn default() -> Self {
        NavTIMEGPS {
            i_tow: 0,
            f_tow: 0,
            week: 0,
            leap_s: 0,
            valid: 0,
            t_acc: 0,
        }
    }
}

impl ros2_client::Message for NavTIMEGPS {}
