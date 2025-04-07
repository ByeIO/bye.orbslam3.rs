use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GimbalDeviceAttitudeStatus {
    pub header: crate::std_msgs::msg::Header,
    pub target_system: u8,
    pub target_component: u8,
    pub flags: u16,
    pub q: crate::geometry_msgs::msg::Quaternion,
    pub angular_velocity_x: f32,
    pub angular_velocity_y: f32,
    pub angular_velocity_z: f32,
    pub failure_flags: u32,
}

impl GimbalDeviceAttitudeStatus {
    pub const FLAGS_RETRACT: u16 = 1;
    pub const FLAGS_NEUTRAL: u16 = 2;
    pub const FLAGS_ROLL_LOCK: u16 = 4;
    pub const FLAGS_PITCH_LOCK: u16 = 8;
    pub const FLAGS_YAW_LOCK: u16 = 16;
    pub const ERROR_FLAGS_AT_ROLL_LIMIT: u32 = 1;
    pub const ERROR_FLAGS_AT_PITCH_LIMIT: u32 = 2;
    pub const ERROR_FLAGS_AT_YAW_LIMIT: u32 = 4;
    pub const ERROR_FLAGS_ENCODER_ERROR: u32 = 8;
    pub const ERROR_FLAGS_POWER_ERROR: u32 = 16;
    pub const ERROR_FLAGS_MOTOR_ERROR: u32 = 32;
    pub const ERROR_FLAGS_SOFTWARE_ERROR: u32 = 64;
    pub const ERROR_FLAGS_COMMS_ERROR: u32 = 128;
    pub const ERROR_FLAGS_CALIBRATION_RUNNING: u32 = 256;
}

impl Default for GimbalDeviceAttitudeStatus {
    fn default() -> Self {
        GimbalDeviceAttitudeStatus {
            header: crate::std_msgs::msg::Header::default(),
            target_system: 0,
            target_component: 0,
            flags: 0,
            q: crate::geometry_msgs::msg::Quaternion::default(),
            angular_velocity_x: 0.0,
            angular_velocity_y: 0.0,
            angular_velocity_z: 0.0,
            failure_flags: 0,
        }
    }
}

impl ros2_client::Message for GimbalDeviceAttitudeStatus {}
