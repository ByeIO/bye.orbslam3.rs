use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScanMatchingStatus {
    pub header: crate::std_msgs::msg::Header,
    pub has_converged: bool,
    pub matching_error: f32,
    pub inlier_fraction: f32,
    pub relative_pose: crate::geometry_msgs::msg::Pose,
    pub prediction_labels: Vec<crate::std_msgs::msg::String>,
    pub prediction_errors: Vec<crate::geometry_msgs::msg::Pose>,
}

impl Default for ScanMatchingStatus {
    fn default() -> Self {
        ScanMatchingStatus {
            header: crate::std_msgs::msg::Header::default(),
            has_converged: false,
            matching_error: 0.0,
            inlier_fraction: 0.0,
            relative_pose: crate::geometry_msgs::msg::Pose::default(),
            prediction_labels: Vec::new(),
            prediction_errors: Vec::new(),
        }
    }
}

impl ros2_client::Message for ScanMatchingStatus {}
