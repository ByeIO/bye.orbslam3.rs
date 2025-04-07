use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RGBDImage {
    pub header: crate::std_msgs::msg::Header,
    pub rgb_camera_info: crate::sensor_msgs::msg::CameraInfo,
    pub depth_camera_info: crate::sensor_msgs::msg::CameraInfo,
    pub rgb: crate::sensor_msgs::msg::Image,
    pub depth: crate::sensor_msgs::msg::Image,
    pub rgb_compressed: crate::sensor_msgs::msg::CompressedImage,
    pub depth_compressed: crate::sensor_msgs::msg::CompressedImage,
    pub key_points: Vec<crate::rtabmap_msgs::msg::KeyPoint>,
    pub points: Vec<crate::rtabmap_msgs::msg::Point3f>,
    pub descriptors: Vec<u8>,
    pub global_descriptor: crate::rtabmap_msgs::msg::GlobalDescriptor,
}

impl Default for RGBDImage {
    fn default() -> Self {
        RGBDImage {
            header: crate::std_msgs::msg::Header::default(),
            rgb_camera_info: crate::sensor_msgs::msg::CameraInfo::default(),
            depth_camera_info: crate::sensor_msgs::msg::CameraInfo::default(),
            rgb: crate::sensor_msgs::msg::Image::default(),
            depth: crate::sensor_msgs::msg::Image::default(),
            rgb_compressed: crate::sensor_msgs::msg::CompressedImage::default(),
            depth_compressed: crate::sensor_msgs::msg::CompressedImage::default(),
            key_points: Vec::new(),
            points: Vec::new(),
            descriptors: Vec::new(),
            global_descriptor: crate::rtabmap_msgs::msg::GlobalDescriptor::default(),
        }
    }
}

impl ros2_client::Message for RGBDImage {}
