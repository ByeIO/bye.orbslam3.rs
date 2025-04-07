use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MeasxData {
    pub gnss_id: u8,
    pub sv_id: u8,
    pub c_no: u8,
    pub mpath_indic: u8,
    pub doppler_ms: i32,
    pub doppler_hz: i32,
    pub whole_chips: u16,
    pub frac_chips: u16,
    pub code_phase: u32,
    pub int_code_phase: u8,
    pub pseu_range_rms_err: u8,
}

impl MeasxData {
    pub const MPATH_NOT_MEASURED: u8 = 0;
    pub const MPATH_LOW: u8 = 1;
    pub const MPATH_MEDIUM: u8 = 2;
    pub const MPATH_HIGH: u8 = 3;
}

impl Default for MeasxData {
    fn default() -> Self {
        MeasxData {
            gnss_id: 0,
            sv_id: 0,
            c_no: 0,
            mpath_indic: 0,
            doppler_ms: 0,
            doppler_hz: 0,
            whole_chips: 0,
            frac_chips: 0,
            code_phase: 0,
            int_code_phase: 0,
            pseu_range_rms_err: 0,
        }
    }
}

impl ros2_client::Message for MeasxData {}
