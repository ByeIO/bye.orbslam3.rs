use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AckermannDrive {
    pub steering_angle: f32,
    pub steering_angle_velocity: f32,
    pub speed: f32,
    pub acceleration: f32,
    pub jerk: f32,
}

impl Default for AckermannDrive {
    fn default() -> Self {
        AckermannDrive {
            steering_angle: 0.0,
            steering_angle_velocity: 0.0,
            speed: 0.0,
            acceleration: 0.0,
            jerk: 0.0,
        }
    }
}

impl ros2_client::Message for AckermannDrive {}
