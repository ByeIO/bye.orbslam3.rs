use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RawxData {
    pub pr_mes: f64,
    pub cp_mes: f64,
    pub do_mes: f32,
    pub gnss_id: u8,
    pub sv_id: u8,
    pub sig_id: u8,
    pub freq_id: u8,
    pub locktime: u16,
    pub c_no: u8,
    pub pr_stdev: u8,
    pub cp_stdev: u8,
    pub do_stdev: u8,
    pub trk_stat: crate::ublox_ubx_msgs::msg::TrkStat,
}

impl Default for RawxData {
    fn default() -> Self {
        RawxData {
            pr_mes: 0.0,
            cp_mes: 0.0,
            do_mes: 0.0,
            gnss_id: 0,
            sv_id: 0,
            sig_id: 0,
            freq_id: 0,
            locktime: 0,
            c_no: 0,
            pr_stdev: 0,
            cp_stdev: 0,
            do_stdev: 0,
            trk_stat: crate::ublox_ubx_msgs::msg::TrkStat::default(),
        }
    }
}

impl ros2_client::Message for RawxData {}
