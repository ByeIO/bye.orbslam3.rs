use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DetectionInfo {
    pub header: crate::std_msgs::msg::Header,
    pub ids: Vec<crate::std_msgs::msg::Int32>,
    pub widths: Vec<crate::std_msgs::msg::Int32>,
    pub heights: Vec<crate::std_msgs::msg::Int32>,
    pub file_paths: Vec<crate::std_msgs::msg::String>,
    pub inliers: Vec<crate::std_msgs::msg::Int32>,
    pub outliers: Vec<crate::std_msgs::msg::Int32>,
    pub homographies: Vec<crate::std_msgs::msg::Float32MultiArray>,
}

impl Default for DetectionInfo {
    fn default() -> Self {
        DetectionInfo {
            header: crate::std_msgs::msg::Header::default(),
            ids: Vec::new(),
            widths: Vec::new(),
            heights: Vec::new(),
            file_paths: Vec::new(),
            inliers: Vec::new(),
            outliers: Vec::new(),
            homographies: Vec::new(),
        }
    }
}

impl ros2_client::Message for DetectionInfo {}
