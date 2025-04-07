use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MeasEpoch {
    pub header: crate::std_msgs::msg::Header,
    pub block_header: crate::septentrio_gnss_driver::msg::BlockHeader,
    pub n: u8,
    pub sb1_length: u8,
    pub sb2_length: u8,
    pub common_flags: u8,
    pub cum_clk_jumps: u8,
    pub type1: Vec<crate::septentrio_gnss_driver::msg::MeasEpochChannelType1>,
}

impl Default for MeasEpoch {
    fn default() -> Self {
        MeasEpoch {
            header: crate::std_msgs::msg::Header::default(),
            block_header: crate::septentrio_gnss_driver::msg::BlockHeader::default(),
            n: 0,
            sb1_length: 0,
            sb2_length: 0,
            common_flags: 0,
            cum_clk_jumps: 0,
            type1: Vec::new(),
        }
    }
}

impl ros2_client::Message for MeasEpoch {}
