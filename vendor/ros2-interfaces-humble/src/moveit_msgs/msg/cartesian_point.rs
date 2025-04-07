use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CartesianPoint {
    pub pose: crate::geometry_msgs::msg::Pose,
    pub velocity: crate::geometry_msgs::msg::Twist,
    pub acceleration: crate::geometry_msgs::msg::Accel,
}

impl Default for CartesianPoint {
    fn default() -> Self {
        CartesianPoint {
            pose: crate::geometry_msgs::msg::Pose::default(),
            velocity: crate::geometry_msgs::msg::Twist::default(),
            acceleration: crate::geometry_msgs::msg::Accel::default(),
        }
    }
}

impl ros2_client::Message for CartesianPoint {}
