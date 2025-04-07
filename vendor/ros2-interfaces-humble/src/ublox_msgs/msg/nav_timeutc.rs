use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NavTIMEUTC {
    pub i_tow: u32,
    pub t_acc: u32,
    pub nano: i32,
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub min: u8,
    pub sec: u8,
    pub valid: u8,
}

impl NavTIMEUTC {
    pub const CLASS_ID: u8 = 1;
    pub const MESSAGE_ID: u8 = 33;
    pub const VALID_TOW: u8 = 1;
    pub const VALID_WKN: u8 = 2;
    pub const VALID_UTC: u8 = 4;
    pub const VALID_UTC_STANDARD_MASK: u8 = 240;
    pub const UTC_STANDARD_NOT_AVAILABLE: u8 = 0;
    pub const UTC_STANDARD_CRL: u8 = 16;
    pub const UTC_STANDARD_NIST: u8 = 32;
    pub const UTC_STANDARD_USNO: u8 = 48;
    pub const UTC_STANDARD_BIPM: u8 = 64;
    pub const UTC_STANDARD_EL: u8 = 80;
    pub const UTC_STANDARD_SU: u8 = 96;
    pub const UTC_STANDARD_NTSC: u8 = 112;
    pub const UTC_STANDARD_UNKNOWN: u8 = 240;
}

impl Default for NavTIMEUTC {
    fn default() -> Self {
        NavTIMEUTC {
            i_tow: 0,
            t_acc: 0,
            nano: 0,
            year: 0,
            month: 0,
            day: 0,
            hour: 0,
            min: 0,
            sec: 0,
            valid: 0,
        }
    }
}

impl ros2_client::Message for NavTIMEUTC {}
