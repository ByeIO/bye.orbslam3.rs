use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArucoDetection {
    pub header: crate::std_msgs::msg::Header,
    pub markers: Vec<crate::aruco_opencv_msgs::msg::MarkerPose>,
    pub boards: Vec<crate::aruco_opencv_msgs::msg::BoardPose>,
}

impl Default for ArucoDetection {
    fn default() -> Self {
        ArucoDetection {
            header: crate::std_msgs::msg::Header::default(),
            markers: Vec::new(),
            boards: Vec::new(),
        }
    }
}

impl ros2_client::Message for ArucoDetection {}
