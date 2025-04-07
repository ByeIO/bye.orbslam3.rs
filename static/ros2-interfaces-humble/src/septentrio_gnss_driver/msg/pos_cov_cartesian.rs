use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PosCovCartesian {
    pub header: crate::std_msgs::msg::Header,
    pub block_header: crate::septentrio_gnss_driver::msg::BlockHeader,
    pub mode: u8,
    pub error: u8,
    pub cov_xx: f32,
    pub cov_yy: f32,
    pub cov_zz: f32,
    pub cov_bb: f32,
    pub cov_xy: f32,
    pub cov_xz: f32,
    pub cov_xb: f32,
    pub cov_yz: f32,
    pub cov_yb: f32,
    pub cov_zb: f32,
}

impl Default for PosCovCartesian {
    fn default() -> Self {
        PosCovCartesian {
            header: crate::std_msgs::msg::Header::default(),
            block_header: crate::septentrio_gnss_driver::msg::BlockHeader::default(),
            mode: 0,
            error: 0,
            cov_xx: 0.0,
            cov_yy: 0.0,
            cov_zz: 0.0,
            cov_bb: 0.0,
            cov_xy: 0.0,
            cov_xz: 0.0,
            cov_xb: 0.0,
            cov_yz: 0.0,
            cov_yb: 0.0,
            cov_zb: 0.0,
        }
    }
}

impl ros2_client::Message for PosCovCartesian {}
