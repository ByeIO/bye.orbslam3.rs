use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BmsState {
    pub version_h: u8,
    pub version_l: u8,
    pub bms_status: u8,
    pub soc: u8,
    pub current: i32,
    pub cycle: u16,
    pub bq_ntc: [i8; 2],
    pub mcu_ntc: [i8; 2],
    pub cell_vol: [u16; 10],
}

impl Default for BmsState {
    fn default() -> Self {
        BmsState {
            version_h: 0,
            version_l: 0,
            bms_status: 0,
            soc: 0,
            current: 0,
            cycle: 0,
            bq_ntc: [0; 2],
            mcu_ntc: [0; 2],
            cell_vol: [0; 10],
        }
    }
}

impl ros2_client::Message for BmsState {}
