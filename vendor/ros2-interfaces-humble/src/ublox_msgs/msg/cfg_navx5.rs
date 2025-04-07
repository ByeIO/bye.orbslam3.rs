use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CfgNAVX5 {
    pub version: u16,
    pub mask1: u16,
    pub mask2: u32,
    pub reserved1: [u8; 2],
    pub min_svs: u8,
    pub max_svs: u8,
    pub min_cno: u8,
    pub reserved2: u8,
    pub ini_fix3d: u8,
    pub reserved3: [u8; 2],
    pub ack_aiding: u8,
    pub wkn_rollover: u16,
    pub sig_atten_comp_mode: u8,
    pub reserved4: [u8; 5],
    pub use_ppp: u8,
    pub aop_cfg: u8,
    pub reserved5: [u8; 2],
    pub aop_orb_max_err: u16,
    pub reserved6: [u8; 7],
    pub use_adr: u8,
}

impl CfgNAVX5 {
    pub const CLASS_ID: u8 = 6;
    pub const MESSAGE_ID: u8 = 35;
    pub const MASK1_MIN_MAX: u16 = 4;
    pub const MASK1_MIN_CNO: u16 = 8;
    pub const MASK1_INITIAL_FIX_3D: u16 = 64;
    pub const MASK1_WKN_ROLL: u16 = 512;
    pub const MASK1_ACK_AID: u16 = 1024;
    pub const MASK1_PPP: u16 = 8192;
    pub const MASK1_AOP: u16 = 16384;
    pub const MASK2_ADR: u32 = 64;
    pub const MASK2_SIG_ATTEN_COMP_MODE: u32 = 128;
}

impl Default for CfgNAVX5 {
    fn default() -> Self {
        CfgNAVX5 {
            version: 0,
            mask1: 0,
            mask2: 0,
            reserved1: [0; 2],
            min_svs: 0,
            max_svs: 0,
            min_cno: 0,
            reserved2: 0,
            ini_fix3d: 0,
            reserved3: [0; 2],
            ack_aiding: 0,
            wkn_rollover: 0,
            sig_atten_comp_mode: 0,
            reserved4: [0; 5],
            use_ppp: 0,
            aop_cfg: 0,
            reserved5: [0; 2],
            aop_orb_max_err: 0,
            reserved6: [0; 7],
            use_adr: 0,
        }
    }
}

impl ros2_client::Message for CfgNAVX5 {}
