use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UBXNavCov {
    pub header: crate::std_msgs::msg::Header,
    pub itow: u32,
    pub version: u8,
    pub pos_cor_valid: bool,
    pub vel_cor_valid: bool,
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

impl Default for UBXNavCov {
    fn default() -> Self {
        UBXNavCov {
            header: crate::std_msgs::msg::Header::default(),
            itow: 0,
            version: 0,
            pos_cor_valid: false,
            vel_cor_valid: false,
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

impl ros2_client::Message for UBXNavCov {}
