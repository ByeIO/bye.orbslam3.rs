use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SonarDetections {
    pub header: crate::std_msgs::msg::Header,
    pub ping_info: crate::marine_acoustic_msgs::msg::PingInfo,
    pub flags: Vec<crate::marine_acoustic_msgs::msg::DetectionFlag>,
    pub two_way_travel_times: Vec<f32>,
    pub tx_delays: Vec<f32>,
    pub intensities: Vec<f32>,
    pub tx_angles: Vec<f32>,
    pub rx_angles: Vec<f32>,
}

impl Default for SonarDetections {
    fn default() -> Self {
        SonarDetections {
            header: crate::std_msgs::msg::Header::default(),
            ping_info: crate::marine_acoustic_msgs::msg::PingInfo::default(),
            flags: Vec::new(),
            two_way_travel_times: Vec::new(),
            tx_delays: Vec::new(),
            intensities: Vec::new(),
            tx_angles: Vec::new(),
            rx_angles: Vec::new(),
        }
    }
}

impl ros2_client::Message for SonarDetections {}
