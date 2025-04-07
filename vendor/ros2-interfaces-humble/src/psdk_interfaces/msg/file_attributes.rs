use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileAttributes {
    pub photo_ratio: u32,
    pub photo_rotation: u32,
    pub video_duration: u32,
    pub video_frame_rate: u32,
    pub video_rotation: u32,
    pub video_resolution: u32,
}

impl Default for FileAttributes {
    fn default() -> Self {
        FileAttributes {
            photo_ratio: 0,
            photo_rotation: 0,
            video_duration: 0,
            video_frame_rate: 0,
            video_rotation: 0,
            video_resolution: 0,
        }
    }
}

impl ros2_client::Message for FileAttributes {}
