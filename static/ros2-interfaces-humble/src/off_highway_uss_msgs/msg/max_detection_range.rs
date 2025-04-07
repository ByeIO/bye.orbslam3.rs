use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MaxDetectionRange {
    pub header: crate::std_msgs::msg::Header,
    pub max_detection_ranges: [f32; 12],
}

impl Default for MaxDetectionRange {
    fn default() -> Self {
        MaxDetectionRange {
            header: crate::std_msgs::msg::Header::default(),
            max_detection_ranges: [0.0; 12],
        }
    }
}

impl ros2_client::Message for MaxDetectionRange {}
