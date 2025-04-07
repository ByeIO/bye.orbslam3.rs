use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FourWheelSteering {
    pub front_steering_angle: f32,
    pub rear_steering_angle: f32,
    pub front_steering_angle_velocity: f32,
    pub rear_steering_angle_velocity: f32,
    pub speed: f32,
    pub acceleration: f32,
    pub jerk: f32,
}

impl Default for FourWheelSteering {
    fn default() -> Self {
        FourWheelSteering {
            front_steering_angle: 0.0,
            rear_steering_angle: 0.0,
            front_steering_angle_velocity: 0.0,
            rear_steering_angle_velocity: 0.0,
            speed: 0.0,
            acceleration: 0.0,
            jerk: 0.0,
        }
    }
}

impl ros2_client::Message for FourWheelSteering {}
