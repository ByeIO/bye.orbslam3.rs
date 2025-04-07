use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PathPoint {
    pub pose: crate::geometry_msgs::msg::Pose,
    pub longitudinal_velocity_mps: f32,
    pub lateral_velocity_mps: f32,
    pub heading_rate_rps: f32,
    pub is_final: bool,
}

impl Default for PathPoint {
    fn default() -> Self {
        PathPoint {
            pose: crate::geometry_msgs::msg::Pose::default(),
            longitudinal_velocity_mps: 0.0,
            lateral_velocity_mps: 0.0,
            heading_rate_rps: 0.0,
            is_final: false,
        }
    }
}

impl ros2_client::Message for PathPoint {}
