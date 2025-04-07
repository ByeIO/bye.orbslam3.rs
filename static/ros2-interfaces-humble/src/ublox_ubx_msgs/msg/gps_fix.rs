use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GpsFix {
    pub fix_type: u8,
}

impl GpsFix {
    pub const GPS_NO_FIX: u8 = 0;
    pub const GPS_DEAD_RECKONING_ONLY: u8 = 1;
    pub const GPS_FIX_2D: u8 = 2;
    pub const GPS_FIX_3D: u8 = 3;
    pub const GPS_PLUS_DEAD_RECKONING: u8 = 4;
    pub const GPS_TIME_ONLY: u8 = 5;
}

impl Default for GpsFix {
    fn default() -> Self {
        GpsFix {
            fix_type: 0,
        }
    }
}

impl ros2_client::Message for GpsFix {}
