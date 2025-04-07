use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrajectoryPoint {
    pub position: crate::geometry_msgs::msg::Vector3,
    pub twist: crate::geometry_msgs::msg::Vector3,
    pub acceleration: crate::geometry_msgs::msg::Vector3,
    pub yaw_angle: f32,
}

impl Default for TrajectoryPoint {
    fn default() -> Self {
        TrajectoryPoint {
            position: crate::geometry_msgs::msg::Vector3::default(),
            twist: crate::geometry_msgs::msg::Vector3::default(),
            acceleration: crate::geometry_msgs::msg::Vector3::default(),
            yaw_angle: 0.0,
        }
    }
}

impl ros2_client::Message for TrajectoryPoint {}
