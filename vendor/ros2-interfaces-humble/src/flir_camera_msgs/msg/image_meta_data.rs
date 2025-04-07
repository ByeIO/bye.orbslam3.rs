use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImageMetaData {
    pub header: crate::std_msgs::msg::Header,
    pub camera_time: u64,
    pub brightness: i16,
    pub exposure_time: u32,
    pub max_exposure_time: u32,
    pub gain: f32,
}

impl Default for ImageMetaData {
    fn default() -> Self {
        ImageMetaData {
            header: crate::std_msgs::msg::Header::default(),
            camera_time: 0,
            brightness: 0,
            exposure_time: 0,
            max_exposure_time: 0,
            gain: 0.0,
        }
    }
}

impl ros2_client::Message for ImageMetaData {}
