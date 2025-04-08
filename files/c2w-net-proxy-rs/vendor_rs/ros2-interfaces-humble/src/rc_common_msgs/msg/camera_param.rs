use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CameraParam {
    pub header: crate::std_msgs::msg::Header,
    pub is_color_camera: bool,
    pub exposure_time: f32,
    pub gain: f32,
    pub line_status_all: u32,
    pub line_source: Vec<crate::rc_common_msgs::msg::KeyValue>,
    pub extra_data: Vec<crate::rc_common_msgs::msg::KeyValue>,
}

impl Default for CameraParam {
    fn default() -> Self {
        CameraParam {
            header: crate::std_msgs::msg::Header::default(),
            is_color_camera: false,
            exposure_time: 0.0,
            gain: 0.0,
            line_status_all: 0,
            line_source: Vec::new(),
            extra_data: Vec::new(),
        }
    }
}

impl ros2_client::Message for CameraParam {}
