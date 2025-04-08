use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GimbalDeviceSetAttitude {
    pub target_system: u8,
    pub target_component: u8,
    pub flags: u16,
    pub q: crate::geometry_msgs::msg::Quaternion,
    pub angular_velocity_x: f32,
    pub angular_velocity_y: f32,
    pub angular_velocity_z: f32,
}

impl GimbalDeviceSetAttitude {
    pub const FLAGS_RETRACT: u16 = 1;
    pub const FLAGS_NEUTRAL: u16 = 2;
    pub const FLAGS_ROLL_LOCK: u16 = 4;
    pub const FLAGS_PITCH_LOCK: u16 = 8;
    pub const FLAGS_YAW_LOCK: u16 = 16;
}

impl Default for GimbalDeviceSetAttitude {
    fn default() -> Self {
        GimbalDeviceSetAttitude {
            target_system: 0,
            target_component: 0,
            flags: 0,
            q: crate::geometry_msgs::msg::Quaternion::default(),
            angular_velocity_x: 0.0,
            angular_velocity_y: 0.0,
            angular_velocity_z: 0.0,
        }
    }
}

impl ros2_client::Message for GimbalDeviceSetAttitude {}
