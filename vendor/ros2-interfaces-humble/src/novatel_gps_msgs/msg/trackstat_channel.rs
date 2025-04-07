use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrackstatChannel {
    pub prn: i16,
    pub glofreq: i16,
    pub ch_tr_status: u32,
    pub psr: f64,
    pub doppler: f32,
    pub c_no: f32,
    pub locktime: f32,
    pub psr_res: f32,
    pub reject: ::std::string::String,
    pub psr_weight: f32,
}

impl Default for TrackstatChannel {
    fn default() -> Self {
        TrackstatChannel {
            prn: 0,
            glofreq: 0,
            ch_tr_status: 0,
            psr: 0.0,
            doppler: 0.0,
            c_no: 0.0,
            locktime: 0.0,
            psr_res: 0.0,
            reject: ::std::string::String::new(),
            psr_weight: 0.0,
        }
    }
}

impl ros2_client::Message for TrackstatChannel {}
