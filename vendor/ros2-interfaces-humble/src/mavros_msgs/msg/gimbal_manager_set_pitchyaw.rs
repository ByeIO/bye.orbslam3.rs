use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GimbalManagerSetPitchyaw {
    pub target_system: u8,
    pub target_component: u8,
    pub flags: u32,
    pub gimbal_device_id: u8,
    pub pitch: f32,
    pub yaw: f32,
    pub pitch_rate: f32,
    pub yaw_rate: f32,
}

impl GimbalManagerSetPitchyaw {
    pub const GIMBAL_MANAGER_FLAGS_RETRACT: u32 = 1;
    pub const GIMBAL_MANAGER_FLAGS_NEUTRAL: u32 = 2;
    pub const GIMBAL_MANAGER_FLAGS_ROLL_LOCK: u32 = 4;
    pub const GIMBAL_MANAGER_FLAGS_PITCH_LOCK: u32 = 8;
    pub const GIMBAL_MANAGER_FLAGS_YAW_LOCK: u32 = 16;
}

impl Default for GimbalManagerSetPitchyaw {
    fn default() -> Self {
        GimbalManagerSetPitchyaw {
            target_system: 0,
            target_component: 0,
            flags: 0,
            gimbal_device_id: 0,
            pitch: 0.0,
            yaw: 0.0,
            pitch_rate: 0.0,
            yaw_rate: 0.0,
        }
    }
}

impl ros2_client::Message for GimbalManagerSetPitchyaw {}
