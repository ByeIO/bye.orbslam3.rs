use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Transform {
    pub translation: crate::geometry_msgs::msg::Vector3,
    pub rotation: crate::geometry_msgs::msg::Quaternion,
}

impl Default for Transform {
    fn default() -> Self {
        Transform {
            translation: crate::geometry_msgs::msg::Vector3::default(),
            rotation: crate::geometry_msgs::msg::Quaternion::default(),
        }
    }
}

impl ros2_client::Message for Transform {}
