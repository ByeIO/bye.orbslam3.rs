use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WheelOdom {
    pub stamp: crate::builtin_interfaces::msg::Time,
    pub velocity_lin: f32,
    pub velocity_ang: f32,
    pub pose_x: f32,
    pub pose_y: f32,
    pub pose_yaw: f32,
}

impl Default for WheelOdom {
    fn default() -> Self {
        WheelOdom {
            stamp: crate::builtin_interfaces::msg::Time::default(),
            velocity_lin: 0.0,
            velocity_ang: 0.0,
            pose_x: 0.0,
            pose_y: 0.0,
            pose_yaw: 0.0,
        }
    }
}

impl ros2_client::Message for WheelOdom {}
