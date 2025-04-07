use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CameraImageCaptured {
    pub header: crate::std_msgs::msg::Header,
    pub orientation: crate::geometry_msgs::msg::Quaternion,
    pub geo: crate::geographic_msgs::msg::GeoPoint,
    pub relative_alt: f32,
    pub image_index: i32,
    pub capture_result: i8,
    pub file_url: ::std::string::String,
}

impl Default for CameraImageCaptured {
    fn default() -> Self {
        CameraImageCaptured {
            header: crate::std_msgs::msg::Header::default(),
            orientation: crate::geometry_msgs::msg::Quaternion::default(),
            geo: crate::geographic_msgs::msg::GeoPoint::default(),
            relative_alt: 0.0,
            image_index: 0,
            capture_result: 0,
            file_url: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for CameraImageCaptured {}
