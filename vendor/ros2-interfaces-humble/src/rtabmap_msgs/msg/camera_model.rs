use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CameraModel {
    pub camera_info: crate::sensor_msgs::msg::CameraInfo,
    pub local_transform: crate::geometry_msgs::msg::Transform,
}

impl Default for CameraModel {
    fn default() -> Self {
        CameraModel {
            camera_info: crate::sensor_msgs::msg::CameraInfo::default(),
            local_transform: crate::geometry_msgs::msg::Transform::default(),
        }
    }
}

impl ros2_client::Message for CameraModel {}
