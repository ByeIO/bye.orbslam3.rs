use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AttCovEuler {
    pub header: crate::std_msgs::msg::Header,
    pub block_header: crate::septentrio_gnss_driver::msg::BlockHeader,
    pub error: u8,
    pub cov_headhead: f32,
    pub cov_pitchpitch: f32,
    pub cov_rollroll: f32,
    pub cov_headpitch: f32,
    pub cov_headroll: f32,
    pub cov_pitchroll: f32,
}

impl Default for AttCovEuler {
    fn default() -> Self {
        AttCovEuler {
            header: crate::std_msgs::msg::Header::default(),
            block_header: crate::septentrio_gnss_driver::msg::BlockHeader::default(),
            error: 0,
            cov_headhead: 0.0,
            cov_pitchpitch: 0.0,
            cov_rollroll: 0.0,
            cov_headpitch: 0.0,
            cov_headroll: 0.0,
            cov_pitchroll: 0.0,
        }
    }
}

impl ros2_client::Message for AttCovEuler {}
