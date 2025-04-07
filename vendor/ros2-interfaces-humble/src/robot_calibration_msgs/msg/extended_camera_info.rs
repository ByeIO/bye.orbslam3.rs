use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExtendedCameraInfo {
    pub camera_info: crate::sensor_msgs::msg::CameraInfo,
    pub parameters: Vec<crate::robot_calibration_msgs::msg::CameraParameter>,
}

impl Default for ExtendedCameraInfo {
    fn default() -> Self {
        ExtendedCameraInfo {
            camera_info: crate::sensor_msgs::msg::CameraInfo::default(),
            parameters: Vec::new(),
        }
    }
}

impl ros2_client::Message for ExtendedCameraInfo {}
