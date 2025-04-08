use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RxmRAWXMeas {
    pub pr_mes: f64,
    pub cp_mes: f64,
    pub do_mes: f32,
    pub gnss_id: u8,
    pub sv_id: u8,
    pub reserved0: u8,
    pub freq_id: u8,
    pub locktime: u16,
    pub cno: i8,
    pub pr_stdev: u8,
    pub cp_stdev: u8,
    pub do_stdev: u8,
    pub trk_stat: u8,
    pub reserved1: u8,
}

impl RxmRAWXMeas {
    pub const TRK_STAT_PR_VALID: u8 = 1;
    pub const TRK_STAT_CP_VALID: u8 = 2;
    pub const TRK_STAT_HALF_CYC: u8 = 4;
    pub const TRK_STAT_SUB_HALF_CYC: u8 = 8;
}

impl Default for RxmRAWXMeas {
    fn default() -> Self {
        RxmRAWXMeas {
            pr_mes: 0.0,
            cp_mes: 0.0,
            do_mes: 0.0,
            gnss_id: 0,
            sv_id: 0,
            reserved0: 0,
            freq_id: 0,
            locktime: 0,
            cno: 0,
            pr_stdev: 0,
            cp_stdev: 0,
            do_stdev: 0,
            trk_stat: 0,
            reserved1: 0,
        }
    }
}

impl ros2_client::Message for RxmRAWXMeas {}
