use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MrrStatusCanVersion {
    pub header: crate::std_msgs::msg::Header,
    pub can_usc_section_compatibility: u16,
    pub can_pcan_minor_mrr: u8,
    pub can_pcan_major_mrr: u8,
}

impl Default for MrrStatusCanVersion {
    fn default() -> Self {
        MrrStatusCanVersion {
            header: crate::std_msgs::msg::Header::default(),
            can_usc_section_compatibility: 0,
            can_pcan_minor_mrr: 0,
            can_pcan_major_mrr: 0,
        }
    }
}

impl ros2_client::Message for MrrStatusCanVersion {}
