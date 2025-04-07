use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RFStatus {
    pub header: crate::std_msgs::msg::Header,
    pub block_header: crate::septentrio_gnss_driver::msg::BlockHeader,
    pub n: u8,
    pub sb_length: u8,
    pub flags: u8,
    pub rfband: Vec<crate::septentrio_gnss_driver::msg::RFBand>,
}

impl Default for RFStatus {
    fn default() -> Self {
        RFStatus {
            header: crate::std_msgs::msg::Header::default(),
            block_header: crate::septentrio_gnss_driver::msg::BlockHeader::default(),
            n: 0,
            sb_length: 0,
            flags: 0,
            rfband: Vec::new(),
        }
    }
}

impl ros2_client::Message for RFStatus {}
