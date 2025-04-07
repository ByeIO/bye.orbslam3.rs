use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SrrFeatureSwVersion {
    pub header: crate::std_msgs::msg::Header,
    pub lcma_sw_version: u8,
    pub cta_sw_version: u8,
}

impl Default for SrrFeatureSwVersion {
    fn default() -> Self {
        SrrFeatureSwVersion {
            header: crate::std_msgs::msg::Header::default(),
            lcma_sw_version: 0,
            cta_sw_version: 0,
        }
    }
}

impl ros2_client::Message for SrrFeatureSwVersion {}
