use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RGBD {
    pub header: crate::std_msgs::msg::Header,
    pub rgb_camera_info: crate::sensor_msgs::msg::CameraInfo,
    pub depth_camera_info: crate::sensor_msgs::msg::CameraInfo,
    pub rgb: crate::sensor_msgs::msg::Image,
    pub depth: crate::sensor_msgs::msg::Image,
}

impl Default for RGBD {
    fn default() -> Self {
        RGBD {
            header: crate::std_msgs::msg::Header::default(),
            rgb_camera_info: crate::sensor_msgs::msg::CameraInfo::default(),
            depth_camera_info: crate::sensor_msgs::msg::CameraInfo::default(),
            rgb: crate::sensor_msgs::msg::Image::default(),
            depth: crate::sensor_msgs::msg::Image::default(),
        }
    }
}

impl ros2_client::Message for RGBD {}
