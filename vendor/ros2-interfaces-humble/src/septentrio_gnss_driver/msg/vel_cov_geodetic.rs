use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VelCovGeodetic {
    pub header: crate::std_msgs::msg::Header,
    pub block_header: crate::septentrio_gnss_driver::msg::BlockHeader,
    pub mode: u8,
    pub error: u8,
    pub cov_vnvn: f32,
    pub cov_veve: f32,
    pub cov_vuvu: f32,
    pub cov_dtdt: f32,
    pub cov_vnve: f32,
    pub cov_vnvu: f32,
    pub cov_vndt: f32,
    pub cov_vevu: f32,
    pub cov_vedt: f32,
    pub cov_vudt: f32,
}

impl Default for VelCovGeodetic {
    fn default() -> Self {
        VelCovGeodetic {
            header: crate::std_msgs::msg::Header::default(),
            block_header: crate::septentrio_gnss_driver::msg::BlockHeader::default(),
            mode: 0,
            error: 0,
            cov_vnvn: 0.0,
            cov_veve: 0.0,
            cov_vuvu: 0.0,
            cov_dtdt: 0.0,
            cov_vnve: 0.0,
            cov_vnvu: 0.0,
            cov_vndt: 0.0,
            cov_vevu: 0.0,
            cov_vedt: 0.0,
            cov_vudt: 0.0,
        }
    }
}

impl ros2_client::Message for VelCovGeodetic {}
