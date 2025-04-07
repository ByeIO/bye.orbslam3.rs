use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CfgRATE {
    pub meas_rate: u16,
    pub nav_rate: u16,
    pub time_ref: u16,
}

impl CfgRATE {
    pub const CLASS_ID: u8 = 6;
    pub const MESSAGE_ID: u8 = 8;
    pub const TIME_REF_UTC: u16 = 0;
    pub const TIME_REF_GPS: u16 = 1;
    pub const TIME_REF_GLONASS: u16 = 2;
    pub const TIME_REF_BEIDOU: u16 = 3;
    pub const TIME_REF_GALILEO: u16 = 4;
}

impl Default for CfgRATE {
    fn default() -> Self {
        CfgRATE {
            meas_rate: 0,
            nav_rate: 0,
            time_ref: 0,
        }
    }
}

impl ros2_client::Message for CfgRATE {}
