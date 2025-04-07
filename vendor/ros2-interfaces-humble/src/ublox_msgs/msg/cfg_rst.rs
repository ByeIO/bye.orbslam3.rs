use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CfgRST {
    pub nav_bbr_mask: u16,
    pub reset_mode: u8,
    pub reserved1: u8,
}

impl CfgRST {
    pub const CLASS_ID: u8 = 6;
    pub const MESSAGE_ID: u8 = 4;
    pub const NAV_BBR_HOT_START: u16 = 0;
    pub const NAV_BBR_WARM_START: u16 = 1;
    pub const NAV_BBR_COLD_START: u16 = 65535;
    pub const NAV_BBR_EPH: u16 = 1;
    pub const NAV_BBR_ALM: u16 = 2;
    pub const NAV_BBR_HEALTH: u16 = 4;
    pub const NAV_BBR_KLOB: u16 = 8;
    pub const NAV_BBR_POS: u16 = 16;
    pub const NAV_BBR_CLKD: u16 = 32;
    pub const NAV_BBR_OSC: u16 = 64;
    pub const NAV_BBR_UTC: u16 = 128;
    pub const NAV_BBR_RTC: u16 = 256;
    pub const NAV_BBR_AOP: u16 = 32768;
    pub const RESET_MODE_HW_IMMEDIATE: u8 = 0;
    pub const RESET_MODE_SW: u8 = 1;
    pub const RESET_MODE_GNSS: u8 = 2;
    pub const RESET_MODE_HW_AFTER_SHUTDOWN: u8 = 4;
    pub const RESET_MODE_GNSS_STOP: u8 = 8;
    pub const RESET_MODE_GNSS_START: u8 = 9;
}

impl Default for CfgRST {
    fn default() -> Self {
        CfgRST {
            nav_bbr_mask: 0,
            reset_mode: 0,
            reserved1: 0,
        }
    }
}

impl ros2_client::Message for CfgRST {}
