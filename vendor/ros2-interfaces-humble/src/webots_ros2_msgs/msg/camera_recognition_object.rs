use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CameraRecognitionObject {
    pub id: i32,
    pub pose: crate::geometry_msgs::msg::PoseStamped,
    pub bbox: crate::vision_msgs::msg::BoundingBox2D,
    pub colors: Vec<crate::std_msgs::msg::ColorRGBA>,
    pub model: ::std::string::String,
}

impl Default for CameraRecognitionObject {
    fn default() -> Self {
        CameraRecognitionObject {
            id: 0,
            pose: crate::geometry_msgs::msg::PoseStamped::default(),
            bbox: crate::vision_msgs::msg::BoundingBox2D::default(),
            colors: Vec::new(),
            model: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for CameraRecognitionObject {}
