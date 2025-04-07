use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MrrStatusSwVersion {
    pub header: crate::std_msgs::msg::Header,
    pub can_pbl_field_revision: u8,
    pub can_pbl_promote_revision: u8,
    pub can_sw_field_revision: u8,
    pub can_sw_promote_revision: u8,
    pub can_sw_release_revision: u8,
    pub can_pbl_release_revision: u8,
}

impl Default for MrrStatusSwVersion {
    fn default() -> Self {
        MrrStatusSwVersion {
            header: crate::std_msgs::msg::Header::default(),
            can_pbl_field_revision: 0,
            can_pbl_promote_revision: 0,
            can_sw_field_revision: 0,
            can_sw_promote_revision: 0,
            can_sw_release_revision: 0,
            can_pbl_release_revision: 0,
        }
    }
}

impl ros2_client::Message for MrrStatusSwVersion {}
