use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CfgTMODE3 {
    pub version: u8,
    pub reserved1: u8,
    pub flags: u16,
    pub ecef_x_or_lat: i32,
    pub ecef_y_or_lon: i32,
    pub ecef_z_or_alt: i32,
    pub ecef_x_or_lat_hp: i8,
    pub ecef_y_or_lon_hp: i8,
    pub ecef_z_or_alt_hp: i8,
    pub reserved2: u8,
    pub fixed_pos_acc: u32,
    pub svin_min_dur: u32,
    pub svin_acc_limit: u32,
    pub reserved3: [u8; 8],
}

impl CfgTMODE3 {
    pub const CLASS_ID: u8 = 6;
    pub const MESSAGE_ID: u8 = 113;
    pub const FLAGS_MODE_MASK: u16 = 255;
    pub const FLAGS_MODE_DISABLED: u16 = 0;
    pub const FLAGS_MODE_SURVEY_IN: u16 = 1;
    pub const FLAGS_MODE_FIXED: u16 = 2;
    pub const FLAGS_LLA: u16 = 256;
}

impl Default for CfgTMODE3 {
    fn default() -> Self {
        CfgTMODE3 {
            version: 0,
            reserved1: 0,
            flags: 0,
            ecef_x_or_lat: 0,
            ecef_y_or_lon: 0,
            ecef_z_or_alt: 0,
            ecef_x_or_lat_hp: 0,
            ecef_y_or_lon_hp: 0,
            ecef_z_or_alt_hp: 0,
            reserved2: 0,
            fixed_pos_acc: 0,
            svin_min_dur: 0,
            svin_acc_limit: 0,
            reserved3: [0; 8],
        }
    }
}

impl ros2_client::Message for CfgTMODE3 {}
