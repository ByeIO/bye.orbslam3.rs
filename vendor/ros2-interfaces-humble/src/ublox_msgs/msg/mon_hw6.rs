use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MonHW6 {
    pub pin_sel: u32,
    pub pin_bank: u32,
    pub pin_dir: u32,
    pub pin_val: u32,
    pub noise_per_ms: u16,
    pub agc_cnt: u16,
    pub a_status: u8,
    pub a_power: u8,
    pub flags: u8,
    pub reserved0: u8,
    pub used_mask: u32,
    pub vp: [u8; 25],
    pub jam_ind: u8,
    pub reserved1: [u8; 2],
    pub pin_irq: u32,
    pub pull_h: u32,
    pub pull_l: u32,
}

impl MonHW6 {
    pub const CLASS_ID: u8 = 10;
    pub const MESSAGE_ID: u8 = 9;
    pub const A_STATUS_INIT: u8 = 0;
    pub const A_STATUS_UNKNOWN: u8 = 1;
    pub const A_STATUS_OK: u8 = 2;
    pub const A_STATUS_SHORT: u8 = 3;
    pub const A_STATUS_OPEN: u8 = 4;
    pub const A_POWER_OFF: u8 = 0;
    pub const A_POWER_ON: u8 = 1;
    pub const A_POWER_UNKNOWN: u8 = 2;
    pub const FLAGS_RTC_CALIB: u8 = 1;
    pub const FLAGS_SAFE_BOOT: u8 = 2;
    pub const FLAGS_JAMMING_STATE_MASK: u8 = 12;
    pub const JAMMING_STATE_UNKNOWN_OR_DISABLED: u8 = 0;
    pub const JAMMING_STATE_OK: u8 = 4;
    pub const JAMMING_STATE_WARNING: u8 = 8;
    pub const JAMMING_STATE_CRITICAL: u8 = 12;
    pub const FLAGS_XTAL_ABSENT: u8 = 16;
    pub const JAM_IND_NONE: u8 = 0;
    pub const JAM_IND_STRONG: u8 = 255;
}

impl Default for MonHW6 {
    fn default() -> Self {
        MonHW6 {
            pin_sel: 0,
            pin_bank: 0,
            pin_dir: 0,
            pin_val: 0,
            noise_per_ms: 0,
            agc_cnt: 0,
            a_status: 0,
            a_power: 0,
            flags: 0,
            reserved0: 0,
            used_mask: 0,
            vp: [0; 25],
            jam_ind: 0,
            reserved1: [0; 2],
            pin_irq: 0,
            pull_h: 0,
            pull_l: 0,
        }
    }
}

impl ros2_client::Message for MonHW6 {}
