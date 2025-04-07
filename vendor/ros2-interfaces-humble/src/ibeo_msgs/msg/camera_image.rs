use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CameraImage {
    pub header: crate::std_msgs::msg::Header,
    pub ibeo_header: crate::ibeo_msgs::msg::IbeoDataHeader,
    pub image_format: u16,
    pub us_since_power_on: u32,
    pub timestamp: crate::builtin_interfaces::msg::Time,
    pub device_id: u8,
    pub mounting_position: crate::ibeo_msgs::msg::MountingPositionF,
    pub horizontal_opening_angle: f64,
    pub vertical_opening_angle: f64,
    pub image_width: u16,
    pub image_height: u16,
    pub compressed_size: u32,
    pub image_buffer: Vec<u8>,
}

impl CameraImage {
    pub const JPEG: u16 = 0;
    pub const MJPEG: u16 = 1;
    pub const GRAY8: u16 = 2;
    pub const YUV420: u16 = 3;
    pub const YUV422: u16 = 4;
}

impl Default for CameraImage {
    fn default() -> Self {
        CameraImage {
            header: crate::std_msgs::msg::Header::default(),
            ibeo_header: crate::ibeo_msgs::msg::IbeoDataHeader::default(),
            image_format: 0,
            us_since_power_on: 0,
            timestamp: crate::builtin_interfaces::msg::Time::default(),
            device_id: 0,
            mounting_position: crate::ibeo_msgs::msg::MountingPositionF::default(),
            horizontal_opening_angle: 0.0,
            vertical_opening_angle: 0.0,
            image_width: 0,
            image_height: 0,
            compressed_size: 0,
            image_buffer: Vec::new(),
        }
    }
}

impl ros2_client::Message for CameraImage {}
