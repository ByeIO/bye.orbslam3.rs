use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CfgANT {
    pub flags: u16,
    pub pins: u16,
}

impl CfgANT {
    pub const CLASS_ID: u8 = 6;
    pub const MESSAGE_ID: u8 = 19;
    pub const FLAGS_SVCS: u16 = 1;
    pub const FLAGS_SCD: u16 = 2;
    pub const FLAGS_OCD: u16 = 4;
    pub const FLAGS_PDWN_ON_SCD: u16 = 8;
    pub const FLAGS_RECOVERY: u16 = 16;
    pub const PIN_SWITCH_MASK: u16 = 31;
    pub const PIN_SCD_MASK: u16 = 992;
    pub const PIN_OCD_MASK: u16 = 31744;
    pub const PIN_RECONFIG: u16 = 32678;
}

impl Default for CfgANT {
    fn default() -> Self {
        CfgANT {
            flags: 0,
            pins: 0,
        }
    }
}

impl ros2_client::Message for CfgANT {}
