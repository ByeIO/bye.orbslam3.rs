use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PosCovGeodetic {
    pub header: crate::std_msgs::msg::Header,
    pub block_header: crate::septentrio_gnss_driver::msg::BlockHeader,
    pub mode: u8,
    pub error: u8,
    pub cov_latlat: f32,
    pub cov_lonlon: f32,
    pub cov_hgthgt: f32,
    pub cov_bb: f32,
    pub cov_latlon: f32,
    pub cov_lathgt: f32,
    pub cov_latb: f32,
    pub cov_lonhgt: f32,
    pub cov_lonb: f32,
    pub cov_hb: f32,
}

impl Default for PosCovGeodetic {
    fn default() -> Self {
        PosCovGeodetic {
            header: crate::std_msgs::msg::Header::default(),
            block_header: crate::septentrio_gnss_driver::msg::BlockHeader::default(),
            mode: 0,
            error: 0,
            cov_latlat: 0.0,
            cov_lonlon: 0.0,
            cov_hgthgt: 0.0,
            cov_bb: 0.0,
            cov_latlon: 0.0,
            cov_lathgt: 0.0,
            cov_latb: 0.0,
            cov_lonhgt: 0.0,
            cov_lonb: 0.0,
            cov_hb: 0.0,
        }
    }
}

impl ros2_client::Message for PosCovGeodetic {}
