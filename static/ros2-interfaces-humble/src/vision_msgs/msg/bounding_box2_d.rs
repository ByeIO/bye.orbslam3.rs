use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BoundingBox2D {
    pub center: crate::vision_msgs::msg::Pose2D,
    pub size_x: f64,
    pub size_y: f64,
}

impl Default for BoundingBox2D {
    fn default() -> Self {
        BoundingBox2D {
            center: crate::vision_msgs::msg::Pose2D::default(),
            size_x: 0.0,
            size_y: 0.0,
        }
    }
}

impl ros2_client::Message for BoundingBox2D {}
