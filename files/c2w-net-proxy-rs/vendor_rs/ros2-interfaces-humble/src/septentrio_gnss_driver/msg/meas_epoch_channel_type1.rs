use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MeasEpochChannelType1 {
    pub rx_channel: u8,
    #[serde(rename = "type")]    pub type_: u8,
    pub sv_id: u8,
    pub misc: u8,
    pub code_lsb: u32,
    pub doppler: i32,
    pub carrier_lsb: u16,
    pub carrier_msb: i8,
    pub cn0: u8,
    pub lock_time: u16,
    pub obs_info: u8,
    pub n2: u8,
    pub type2: Vec<crate::septentrio_gnss_driver::msg::MeasEpochChannelType2>,
}

impl Default for MeasEpochChannelType1 {
    fn default() -> Self {
        MeasEpochChannelType1 {
            rx_channel: 0,
            type_: 0,
            sv_id: 0,
            misc: 0,
            code_lsb: 0,
            doppler: 0,
            carrier_lsb: 0,
            carrier_msb: 0,
            cn0: 0,
            lock_time: 0,
            obs_info: 0,
            n2: 0,
            type2: Vec::new(),
        }
    }
}

impl ros2_client::Message for MeasEpochChannelType1 {}
