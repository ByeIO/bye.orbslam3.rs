use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Satellite {
    pub prn: u8,
    pub elevation: u8,
    pub azimuth: u16,
    pub snr: i8,
}

impl Default for Satellite {
    fn default() -> Self {
        Satellite {
            prn: 0,
            elevation: 0,
            azimuth: 0,
            snr: 0,
        }
    }
}

impl ros2_client::Message for Satellite {}
