use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BoundingBox3D {
    pub center: crate::geometry_msgs::msg::Pose,
    pub size: crate::geometry_msgs::msg::Vector3,
}

impl Default for BoundingBox3D {
    fn default() -> Self {
        BoundingBox3D {
            center: crate::geometry_msgs::msg::Pose::default(),
            size: crate::geometry_msgs::msg::Vector3::default(),
        }
    }
}

impl ros2_client::Message for BoundingBox3D {}
