use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NavCOV {
    pub i_tow: u32,
    pub version: u8,
    pub pos_cov_valid: u8,
    pub vel_cov_valid: u8,
    pub reserved_0: [u8; 9],
    pub pos_cov_nn: f32,
    pub pos_cov_ne: f32,
    pub pos_cov_nd: f32,
    pub pos_cov_ee: f32,
    pub pos_cov_ed: f32,
    pub pos_cov_dd: f32,
    pub vel_cov_nn: f32,
    pub vel_cov_ne: f32,
    pub vel_cov_nd: f32,
    pub vel_cov_ee: f32,
    pub vel_cov_ed: f32,
    pub vel_cov_dd: f32,
}

impl NavCOV {
    pub const CLASS_ID: u8 = 1;
    pub const MESSAGE_ID: u8 = 54;
}

impl Default for NavCOV {
    fn default() -> Self {
        NavCOV {
            i_tow: 0,
            version: 0,
            pos_cov_valid: 0,
            vel_cov_valid: 0,
            reserved_0: [0; 9],
            pos_cov_nn: 0.0,
            pos_cov_ne: 0.0,
            pos_cov_nd: 0.0,
            pos_cov_ee: 0.0,
            pos_cov_ed: 0.0,
            pos_cov_dd: 0.0,
            vel_cov_nn: 0.0,
            vel_cov_ne: 0.0,
            vel_cov_nd: 0.0,
            vel_cov_ee: 0.0,
            vel_cov_ed: 0.0,
            vel_cov_dd: 0.0,
        }
    }
}

impl ros2_client::Message for NavCOV {}
