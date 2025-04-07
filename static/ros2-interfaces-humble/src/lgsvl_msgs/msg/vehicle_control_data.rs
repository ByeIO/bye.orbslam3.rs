use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VehicleControlData {
    pub header: crate::std_msgs::msg::Header,
    pub acceleration_pct: f32,
    pub braking_pct: f32,
    pub target_wheel_angle: f32,
    pub target_wheel_angular_rate: f32,
    pub target_gear: u8,
}

impl VehicleControlData {
    pub const GEAR_NEUTRAL: u8 = 0;
    pub const GEAR_DRIVE: u8 = 1;
    pub const GEAR_REVERSE: u8 = 2;
    pub const GEAR_PARKING: u8 = 3;
    pub const GEAR_LOW: u8 = 4;
}

impl Default for VehicleControlData {
    fn default() -> Self {
        VehicleControlData {
            header: crate::std_msgs::msg::Header::default(),
            acceleration_pct: 0.0,
            braking_pct: 0.0,
            target_wheel_angle: 0.0,
            target_wheel_angular_rate: 0.0,
            target_gear: 0,
        }
    }
}

impl ros2_client::Message for VehicleControlData {}
