use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnsParasetToEEPROM {
    pub header: crate::std_msgs::msg::Header,
    pub command: u8,
    pub paraset_cksum_low_byte: u8,
    pub paraset_cksum_high_byte: u8,
}

impl Default for AnsParasetToEEPROM {
    fn default() -> Self {
        AnsParasetToEEPROM {
            header: crate::std_msgs::msg::Header::default(),
            command: 0,
            paraset_cksum_low_byte: 0,
            paraset_cksum_high_byte: 0,
        }
    }
}

impl ros2_client::Message for AnsParasetToEEPROM {}
