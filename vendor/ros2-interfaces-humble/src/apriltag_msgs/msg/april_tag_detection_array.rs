use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AprilTagDetectionArray {
    pub header: crate::std_msgs::msg::Header,
    pub detections: Vec<crate::apriltag_msgs::msg::AprilTagDetection>,
}

impl Default for AprilTagDetectionArray {
    fn default() -> Self {
        AprilTagDetectionArray {
            header: crate::std_msgs::msg::Header::default(),
            detections: Vec::new(),
        }
    }
}

impl ros2_client::Message for AprilTagDetectionArray {}
