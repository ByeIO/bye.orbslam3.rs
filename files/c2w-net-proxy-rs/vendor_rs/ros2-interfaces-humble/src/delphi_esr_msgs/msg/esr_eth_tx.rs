use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[serde_as]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EsrEthTx {
    pub header: crate::std_msgs::msg::Header,
    pub xcp_format_version: u16,
    pub scan_index: u16,
    pub tcp_size: u16,
    pub xcp_scan_type: u8,
    pub look_index: u16,
    pub mmr_scan_index: u16,
    pub target_report_host_speed: f32,
    pub target_report_host_yaw_rate: f32,
    pub xcp_timestamp: u32,
    pub release_revision: u8,
    pub promote_revision: u8,
    pub field_revision: u8,
    pub target_report_count: u8,
    #[serde_as(as = "[_; 64]")]
    pub target_report_range: [f32; 64],
    #[serde_as(as = "[_; 64]")]
    pub target_report_range_rate: [f32; 64],
    #[serde_as(as = "[_; 64]")]
    pub target_report_theta: [f32; 64],
    #[serde_as(as = "[_; 64]")]
    pub target_report_amplitude: [f32; 64],
}

impl Default for EsrEthTx {
    fn default() -> Self {
        EsrEthTx {
            header: crate::std_msgs::msg::Header::default(),
            xcp_format_version: 0,
            scan_index: 0,
            tcp_size: 0,
            xcp_scan_type: 0,
            look_index: 0,
            mmr_scan_index: 0,
            target_report_host_speed: 0.0,
            target_report_host_yaw_rate: 0.0,
            xcp_timestamp: 0,
            release_revision: 0,
            promote_revision: 0,
            field_revision: 0,
            target_report_count: 0,
            target_report_range: [0.0; 64],
            target_report_range_rate: [0.0; 64],
            target_report_theta: [0.0; 64],
            target_report_amplitude: [0.0; 64],
        }
    }
}

impl ros2_client::Message for EsrEthTx {}
