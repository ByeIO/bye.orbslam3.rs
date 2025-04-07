use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MeasEpochChannelType2 {
    #[serde(rename = "type")]    pub type_: u8,
    pub lock_time: u8,
    pub cn0: u8,
    pub offsets_msb: u8,
    pub carrier_msb: i8,
    pub obs_info: u8,
    pub code_offset_lsb: u16,
    pub carrier_lsb: u16,
    pub doppler_offset_lsb: u16,
}

impl Default for MeasEpochChannelType2 {
    fn default() -> Self {
        MeasEpochChannelType2 {
            type_: 0,
            lock_time: 0,
            cn0: 0,
            offsets_msb: 0,
            carrier_msb: 0,
            obs_info: 0,
            code_offset_lsb: 0,
            carrier_lsb: 0,
            doppler_offset_lsb: 0,
        }
    }
}

impl ros2_client::Message for MeasEpochChannelType2 {}
