use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlockHeader {
    pub sync_1: u8, // default: 36
    pub sync_2: u8, // default: 64
    pub crc: u16,
    pub id: u16,
    pub revision: u8,
    pub length: u16,
    pub tow: u32, // default: 4294967295
    pub wnc: u16, // default: 65535
}

impl Default for BlockHeader {
    fn default() -> Self {
        BlockHeader {
            sync_1: 36,
            sync_2: 64,
            crc: 0,
            id: 0,
            revision: 0,
            length: 0,
            tow: 4294967295,
            wnc: 65535,
        }
    }
}

impl ros2_client::Message for BlockHeader {}
