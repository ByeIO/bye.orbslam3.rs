use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BaseVectorGeod {
    pub header: crate::std_msgs::msg::Header,
    pub block_header: crate::septentrio_gnss_driver::msg::BlockHeader,
    pub n: u8,
    pub sb_length: u8,
    pub vector_info_geod: Vec<crate::septentrio_gnss_driver::msg::VectorInfoGeod>,
}

impl Default for BaseVectorGeod {
    fn default() -> Self {
        BaseVectorGeod {
            header: crate::std_msgs::msg::Header::default(),
            block_header: crate::septentrio_gnss_driver::msg::BlockHeader::default(),
            n: 0,
            sb_length: 0,
            vector_info_geod: Vec::new(),
        }
    }
}

impl ros2_client::Message for BaseVectorGeod {}
