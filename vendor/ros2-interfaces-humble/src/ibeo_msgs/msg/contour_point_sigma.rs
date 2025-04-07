use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContourPointSigma {
    pub x: i16,
    pub y: i16,
    pub x_sigma: u8,
    pub y_sigma: u8,
}

impl Default for ContourPointSigma {
    fn default() -> Self {
        ContourPointSigma {
            x: 0,
            y: 0,
            x_sigma: 0,
            y_sigma: 0,
        }
    }
}

impl ros2_client::Message for ContourPointSigma {}
