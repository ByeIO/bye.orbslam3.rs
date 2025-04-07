use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RxmSFRBX {
    pub gnss_id: u8,
    pub sv_id: u8,
    pub reserved0: u8,
    pub freq_id: u8,
    pub num_words: u8,
    pub chn: u8,
    pub version: u8,
    pub reserved1: u8,
    pub dwrd: Vec<u32>,
}

impl RxmSFRBX {
    pub const CLASS_ID: u8 = 2;
    pub const MESSAGE_ID: u8 = 19;
}

impl Default for RxmSFRBX {
    fn default() -> Self {
        RxmSFRBX {
            gnss_id: 0,
            sv_id: 0,
            reserved0: 0,
            freq_id: 0,
            num_words: 0,
            chn: 0,
            version: 0,
            reserved1: 0,
            dwrd: Vec::new(),
        }
    }
}

impl ros2_client::Message for RxmSFRBX {}
