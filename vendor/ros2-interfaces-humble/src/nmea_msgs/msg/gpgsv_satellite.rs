use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GpgsvSatellite {
    pub prn: u8,
    pub elevation: u8,
    pub azimuth: u16,
    pub snr: i8,
}

impl Default for GpgsvSatellite {
    fn default() -> Self {
        GpgsvSatellite {
            prn: 0,
            elevation: 0,
            azimuth: 0,
            snr: 0,
        }
    }
}

impl ros2_client::Message for GpgsvSatellite {}
