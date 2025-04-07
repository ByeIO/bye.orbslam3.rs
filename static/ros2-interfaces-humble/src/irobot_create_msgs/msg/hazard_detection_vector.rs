use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HazardDetectionVector {
    pub header: crate::std_msgs::msg::Header,
    pub detections: Vec<crate::irobot_create_msgs::msg::HazardDetection>,
}

impl Default for HazardDetectionVector {
    fn default() -> Self {
        HazardDetectionVector {
            header: crate::std_msgs::msg::Header::default(),
            detections: Vec::new(),
        }
    }
}

impl ros2_client::Message for HazardDetectionVector {}
