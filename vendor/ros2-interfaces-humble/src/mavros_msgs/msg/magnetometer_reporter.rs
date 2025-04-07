use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MagnetometerReporter {
    pub header: crate::std_msgs::msg::Header,
    pub report: u8,
    pub confidence: f32,
}

impl Default for MagnetometerReporter {
    fn default() -> Self {
        MagnetometerReporter {
            header: crate::std_msgs::msg::Header::default(),
            report: 0,
            confidence: 0.0,
        }
    }
}

impl ros2_client::Message for MagnetometerReporter {}
