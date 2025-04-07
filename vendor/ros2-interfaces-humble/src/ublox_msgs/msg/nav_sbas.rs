use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NavSBAS {
    pub i_tow: u32,
    pub geo: u8,
    pub mode: u8,
    pub sys: i8,
    pub service: u8,
    pub cnt: u8,
    pub reserved0: [u8; 3],
    pub sv: Vec<crate::ublox_msgs::msg::NavSBASSV>,
}

impl NavSBAS {
    pub const CLASS_ID: u8 = 1;
    pub const MESSAGE_ID: u8 = 50;
    pub const MODE_DISABLED: u8 = 0;
    pub const MODE_ENABLED_INTEGRITY: u8 = 1;
    pub const MODE_ENABLED_TESTMODE: u8 = 3;
    pub const SYS_UNKNOWN: i8 = -1;
    pub const SYS_WAAS: i8 = 0;
    pub const SYS_EGNOS: i8 = 1;
    pub const SYS_MSAS: i8 = 2;
    pub const SYS_GAGAN: i8 = 3;
    pub const SYS_GPS: i8 = 16;
    pub const SERVICE_RANGING: u8 = 1;
    pub const SERVICE_CORRECTIONS: u8 = 2;
    pub const SERVICE_INTEGRITY: u8 = 4;
    pub const SERVICE_TESTMODE: u8 = 8;
}

impl Default for NavSBAS {
    fn default() -> Self {
        NavSBAS {
            i_tow: 0,
            geo: 0,
            mode: 0,
            sys: 0,
            service: 0,
            cnt: 0,
            reserved0: [0; 3],
            sv: Vec::new(),
        }
    }
}

impl ros2_client::Message for NavSBAS {}
