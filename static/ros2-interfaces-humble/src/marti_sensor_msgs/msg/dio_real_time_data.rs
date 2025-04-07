use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DioRealTimeData {
    pub header: crate::std_msgs::msg::Header,
    pub sample_frequency: f64,
    pub latest_sample_time: u64,
    pub sample_states: Vec<u16>,
    pub sample_times: Vec<u32>,
}

impl Default for DioRealTimeData {
    fn default() -> Self {
        DioRealTimeData {
            header: crate::std_msgs::msg::Header::default(),
            sample_frequency: 0.0,
            latest_sample_time: 0,
            sample_states: Vec::new(),
            sample_times: Vec::new(),
        }
    }
}

impl ros2_client::Message for DioRealTimeData {}
