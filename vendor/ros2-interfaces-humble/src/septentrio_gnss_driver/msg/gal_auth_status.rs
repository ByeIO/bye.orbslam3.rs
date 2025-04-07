use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GALAuthStatus {
    pub header: crate::std_msgs::msg::Header,
    pub block_header: crate::septentrio_gnss_driver::msg::BlockHeader,
    pub osnma_status: u16,
    pub trusted_time_delta: f32,
    pub gal_active_mask: u64,
    pub gal_authentic_mask: u64,
    pub gps_active_mask: u64,
    pub gps_authentic_mask: u64,
}

impl Default for GALAuthStatus {
    fn default() -> Self {
        GALAuthStatus {
            header: crate::std_msgs::msg::Header::default(),
            block_header: crate::septentrio_gnss_driver::msg::BlockHeader::default(),
            osnma_status: 0,
            trusted_time_delta: 0.0,
            gal_active_mask: 0,
            gal_authentic_mask: 0,
            gps_active_mask: 0,
            gps_authentic_mask: 0,
        }
    }
}

impl ros2_client::Message for GALAuthStatus {}
