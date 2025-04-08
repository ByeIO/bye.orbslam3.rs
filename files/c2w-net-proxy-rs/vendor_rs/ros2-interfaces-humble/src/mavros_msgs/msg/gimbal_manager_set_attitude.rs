use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GimbalManagerSetAttitude {
    pub target_system: u8,
    pub target_component: u8,
    pub flags: u32,
    pub gimbal_device_id: u8,
    pub q: crate::geometry_msgs::msg::Quaternion,
    pub angular_velocity_x: f32,
    pub angular_velocity_y: f32,
    pub angular_velocity_z: f32,
}

impl GimbalManagerSetAttitude {
    pub const GIMBAL_MANAGER_FLAGS_RETRACT: u32 = 1;
    pub const GIMBAL_MANAGER_FLAGS_NEUTRAL: u32 = 2;
    pub const GIMBAL_MANAGER_FLAGS_ROLL_LOCK: u32 = 4;
    pub const GIMBAL_MANAGER_FLAGS_PITCH_LOCK: u32 = 8;
    pub const GIMBAL_MANAGER_FLAGS_YAW_LOCK: u32 = 16;
}

impl Default for GimbalManagerSetAttitude {
    fn default() -> Self {
        GimbalManagerSetAttitude {
            target_system: 0,
            target_component: 0,
            flags: 0,
            gimbal_device_id: 0,
            q: crate::geometry_msgs::msg::Quaternion::default(),
            angular_velocity_x: 0.0,
            angular_velocity_y: 0.0,
            angular_velocity_z: 0.0,
        }
    }
}

impl ros2_client::Message for GimbalManagerSetAttitude {}
