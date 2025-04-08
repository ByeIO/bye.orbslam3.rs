use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PerceptionCameraParameters {
    pub header: crate::std_msgs::msg::Header,
    pub stereo_cameras_direction: u8,
    pub left_intrinsics: [f64; 9],
    pub right_intrinsics: [f64; 9],
    pub rotation_left_in_right: [f64; 9],
    pub translation_left_in_right: [f64; 3],
}

impl Default for PerceptionCameraParameters {
    fn default() -> Self {
        PerceptionCameraParameters {
            header: crate::std_msgs::msg::Header::default(),
            stereo_cameras_direction: 0,
            left_intrinsics: [0.0; 9],
            right_intrinsics: [0.0; 9],
            rotation_left_in_right: [0.0; 9],
            translation_left_in_right: [0.0; 3],
        }
    }
}

impl ros2_client::Message for PerceptionCameraParameters {}
