use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VelCovCartesian {
    pub header: crate::std_msgs::msg::Header,
    pub block_header: crate::septentrio_gnss_driver::msg::BlockHeader,
    pub mode: u8,
    pub error: u8,
    pub cov_vxvx: f32,
    pub cov_vyvy: f32,
    pub cov_vzvz: f32,
    pub cov_dtdt: f32,
    pub cov_vxvy: f32,
    pub cov_vxvz: f32,
    pub cov_vxdt: f32,
    pub cov_vyvz: f32,
    pub cov_vydt: f32,
    pub cov_vzdt: f32,
}

impl Default for VelCovCartesian {
    fn default() -> Self {
        VelCovCartesian {
            header: crate::std_msgs::msg::Header::default(),
            block_header: crate::septentrio_gnss_driver::msg::BlockHeader::default(),
            mode: 0,
            error: 0,
            cov_vxvx: 0.0,
            cov_vyvy: 0.0,
            cov_vzvz: 0.0,
            cov_dtdt: 0.0,
            cov_vxvy: 0.0,
            cov_vxvz: 0.0,
            cov_vxdt: 0.0,
            cov_vyvz: 0.0,
            cov_vydt: 0.0,
            cov_vzdt: 0.0,
        }
    }
}

impl ros2_client::Message for VelCovCartesian {}
