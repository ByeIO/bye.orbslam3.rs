use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CameraModels {
    pub models: Vec<crate::rtabmap_msgs::msg::CameraModel>,
}

impl Default for CameraModels {
    fn default() -> Self {
        CameraModels {
            models: Vec::new(),
        }
    }
}

impl ros2_client::Message for CameraModels {}
