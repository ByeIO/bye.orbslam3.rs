use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RawSonarImage {
    pub header: crate::std_msgs::msg::Header,
    pub ping_info: crate::marine_acoustic_msgs::msg::PingInfo,
    pub sample_rate: f32,
    pub samples_per_beam: u32,
    pub sample0: u32,
    pub tx_delays: Vec<f32>,
    pub tx_angles: Vec<f32>,
    pub rx_angles: Vec<f32>,
    pub image: crate::marine_acoustic_msgs::msg::SonarImageData,
}

impl Default for RawSonarImage {
    fn default() -> Self {
        RawSonarImage {
            header: crate::std_msgs::msg::Header::default(),
            ping_info: crate::marine_acoustic_msgs::msg::PingInfo::default(),
            sample_rate: 0.0,
            samples_per_beam: 0,
            sample0: 0,
            tx_delays: Vec::new(),
            tx_angles: Vec::new(),
            rx_angles: Vec::new(),
            image: crate::marine_acoustic_msgs::msg::SonarImageData::default(),
        }
    }
}

impl ros2_client::Message for RawSonarImage {}
