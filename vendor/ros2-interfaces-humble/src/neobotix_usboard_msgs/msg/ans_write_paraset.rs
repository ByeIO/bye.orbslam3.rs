use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnsWriteParaset {
    pub header: crate::std_msgs::msg::Header,
    pub command: u8,
    pub paramset_cksum_low_byte: u8,
    pub paramset_cksum_high_byte: u8,
}

impl Default for AnsWriteParaset {
    fn default() -> Self {
        AnsWriteParaset {
            header: crate::std_msgs::msg::Header::default(),
            command: 0,
            paramset_cksum_low_byte: 0,
            paramset_cksum_high_byte: 0,
        }
    }
}

impl ros2_client::Message for AnsWriteParaset {}
