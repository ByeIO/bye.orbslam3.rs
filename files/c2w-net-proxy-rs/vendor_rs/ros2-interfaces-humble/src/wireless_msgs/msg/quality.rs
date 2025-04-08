use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Quality {
    pub header: crate::std_msgs::msg::Header,
    pub messages_received: u16,
    pub messages_missed: u16,
    pub total_length: u32,
    pub message_lengths: Vec<u32>,
    pub latency_avg: f32,
    pub latency_measurements: Vec<f32>,
}

impl Default for Quality {
    fn default() -> Self {
        Quality {
            header: crate::std_msgs::msg::Header::default(),
            messages_received: 0,
            messages_missed: 0,
            total_length: 0,
            message_lengths: Vec::new(),
            latency_avg: 0.0,
            latency_measurements: Vec::new(),
        }
    }
}

impl ros2_client::Message for Quality {}
