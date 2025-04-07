use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MgaGAL {
    #[serde(rename = "type")]    pub type_: u8,
    pub version: u8,
    pub svid: u8,
    pub reserved0: u8,
    pub iod_nav: u16,
    pub delta_n: i16,
    pub m0: i32,
    pub e: u32,
    pub sqrt_a: u32,
    pub omega0: i32,
    pub i0: i32,
    pub omega: i32,
    pub omega_dot: i32,
    pub i_dot: i16,
    pub cuc: i16,
    pub cus: i16,
    pub crc: i16,
    pub crs: i16,
    pub cic: i16,
    pub cis: i16,
    pub toe: u16,
    pub af0: i32,
    pub af1: i32,
    pub af2: i8,
    pub sisaindex_e1_e5b: u8,
    pub toc: u16,
    pub bgd_e1_e5b: i16,
    pub reserved1: [u8; 2],
    pub health_e1b: u8,
    pub data_validity_e1b: u8,
    pub health_e5b: u8,
    pub data_validity_e5b: u8,
    pub reserved2: [u8; 4],
}

impl MgaGAL {
    pub const CLASS_ID: u8 = 19;
    pub const MESSAGE_ID: u8 = 2;
}

impl Default for MgaGAL {
    fn default() -> Self {
        MgaGAL {
            type_: 0,
            version: 0,
            svid: 0,
            reserved0: 0,
            iod_nav: 0,
            delta_n: 0,
            m0: 0,
            e: 0,
            sqrt_a: 0,
            omega0: 0,
            i0: 0,
            omega: 0,
            omega_dot: 0,
            i_dot: 0,
            cuc: 0,
            cus: 0,
            crc: 0,
            crs: 0,
            cic: 0,
            cis: 0,
            toe: 0,
            af0: 0,
            af1: 0,
            af2: 0,
            sisaindex_e1_e5b: 0,
            toc: 0,
            bgd_e1_e5b: 0,
            reserved1: [0; 2],
            health_e1b: 0,
            data_validity_e1b: 0,
            health_e5b: 0,
            data_validity_e5b: 0,
            reserved2: [0; 4],
        }
    }
}

impl ros2_client::Message for MgaGAL {}
