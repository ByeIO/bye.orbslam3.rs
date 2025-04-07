use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrajectoryPoint {
    pub time_from_start: crate::builtin_interfaces::msg::Duration,
    pub pose: crate::geometry_msgs::msg::Pose,
    pub longitudinal_velocity_mps: f32,
    pub lateral_velocity_mps: f32,
    pub acceleration_mps2: f32,
    pub heading_rate_rps: f32,
    pub front_wheel_angle_rad: f32,
    pub rear_wheel_angle_rad: f32,
}

impl Default for TrajectoryPoint {
    fn default() -> Self {
        TrajectoryPoint {
            time_from_start: crate::builtin_interfaces::msg::Duration::default(),
            pose: crate::geometry_msgs::msg::Pose::default(),
            longitudinal_velocity_mps: 0.0,
            lateral_velocity_mps: 0.0,
            acceleration_mps2: 0.0,
            heading_rate_rps: 0.0,
            front_wheel_angle_rad: 0.0,
            rear_wheel_angle_rad: 0.0,
        }
    }
}

impl ros2_client::Message for TrajectoryPoint {}
