use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NavVELECEF {
    pub i_tow: u32,
    pub ecef_vx: i32,
    pub ecef_vy: i32,
    pub ecef_vz: i32,
    pub s_acc: u32,
}

impl NavVELECEF {
    pub const CLASS_ID: u8 = 1;
    pub const MESSAGE_ID: u8 = 17;
}

impl Default for NavVELECEF {
    fn default() -> Self {
        NavVELECEF {
            i_tow: 0,
            ecef_vx: 0,
            ecef_vy: 0,
            ecef_vz: 0,
            s_acc: 0,
        }
    }
}

impl ros2_client::Message for NavVELECEF {}
