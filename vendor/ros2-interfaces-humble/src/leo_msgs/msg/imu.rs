use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Imu {
    pub stamp: crate::builtin_interfaces::msg::Time,
    pub temperature: f32,
    pub gyro_x: f32,
    pub gyro_y: f32,
    pub gyro_z: f32,
    pub accel_x: f32,
    pub accel_y: f32,
    pub accel_z: f32,
}

impl Default for Imu {
    fn default() -> Self {
        Imu {
            stamp: crate::builtin_interfaces::msg::Time::default(),
            temperature: 0.0,
            gyro_x: 0.0,
            gyro_y: 0.0,
            gyro_z: 0.0,
            accel_x: 0.0,
            accel_y: 0.0,
            accel_z: 0.0,
        }
    }
}

impl ros2_client::Message for Imu {}
