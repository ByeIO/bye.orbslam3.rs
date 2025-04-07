use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SbgMagStatus {
    pub mag_x: bool,
    pub mag_y: bool,
    pub mag_z: bool,
    pub accel_x: bool,
    pub accel_y: bool,
    pub accel_z: bool,
    pub mags_in_range: bool,
    pub accels_in_range: bool,
    pub calibration: bool,
}

impl Default for SbgMagStatus {
    fn default() -> Self {
        SbgMagStatus {
            mag_x: false,
            mag_y: false,
            mag_z: false,
            accel_x: false,
            accel_y: false,
            accel_z: false,
            mags_in_range: false,
            accels_in_range: false,
            calibration: false,
        }
    }
}

impl ros2_client::Message for SbgMagStatus {}
