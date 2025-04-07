use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SonarRanges {
    pub header: crate::std_msgs::msg::Header,
    pub ping_info: crate::marine_acoustic_msgs::msg::PingInfo,
    pub flags: Vec<crate::marine_acoustic_msgs::msg::DetectionFlag>,
    pub transmit_delays: Vec<f32>,
    pub intensities: Vec<f32>,
    pub beam_unit_vec: Vec<crate::geometry_msgs::msg::Vector3>,
    pub ranges: Vec<f32>,
}

impl Default for SonarRanges {
    fn default() -> Self {
        SonarRanges {
            header: crate::std_msgs::msg::Header::default(),
            ping_info: crate::marine_acoustic_msgs::msg::PingInfo::default(),
            flags: Vec::new(),
            transmit_delays: Vec::new(),
            intensities: Vec::new(),
            beam_unit_vec: Vec::new(),
            ranges: Vec::new(),
        }
    }
}

impl ros2_client::Message for SonarRanges {}
