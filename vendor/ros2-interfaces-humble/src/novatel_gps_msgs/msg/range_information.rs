use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RangeInformation {
    pub prn_number: u16,
    pub glofreq: u16,
    pub psr: f64,
    pub psr_std: f32,
    pub adr: f64,
    pub adr_std: f32,
    pub dopp: f32,
    pub noise_density_ratio: f32,
    pub locktime: f32,
    pub tracking_status: u32,
}

impl Default for RangeInformation {
    fn default() -> Self {
        RangeInformation {
            prn_number: 0,
            glofreq: 0,
            psr: 0.0,
            psr_std: 0.0,
            adr: 0.0,
            adr_std: 0.0,
            dopp: 0.0,
            noise_density_ratio: 0.0,
            locktime: 0.0,
            tracking_status: 0,
        }
    }
}

impl ros2_client::Message for RangeInformation {}
