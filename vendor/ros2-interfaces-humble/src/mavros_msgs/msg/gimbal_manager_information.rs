use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GimbalManagerInformation {
    pub header: crate::std_msgs::msg::Header,
    pub cap_flags: u32,
    pub gimbal_device_id: u8,
    pub roll_min: f32,
    pub roll_max: f32,
    pub pitch_min: f32,
    pub pitch_max: f32,
    pub yaw_min: f32,
    pub yaw_max: f32,
}

impl GimbalManagerInformation {
    pub const CAP_FLAGS_HAS_RETRACT: u32 = 1;
    pub const CAP_FLAGS_HAS_NEUTRAL: u32 = 2;
    pub const CAP_FLAGS_HAS_ROLL_AXIS: u32 = 4;
    pub const CAP_FLAGS_HAS_ROLL_FOLLOW: u32 = 8;
    pub const CAP_FLAGS_HAS_ROLL_LOCK: u32 = 16;
    pub const CAP_FLAGS_HAS_PITCH_AXIS: u32 = 32;
    pub const CAP_FLAGS_HAS_PITCH_FOLLOW: u32 = 64;
    pub const CAP_FLAGS_HAS_PITCH_LOCK: u32 = 128;
    pub const CAP_FLAGS_HAS_YAW_AXIS: u32 = 256;
    pub const CAP_FLAGS_HAS_YAW_FOLLOW: u32 = 512;
    pub const CAP_FLAGS_HAS_YAW_LOCK: u32 = 1024;
    pub const CAP_FLAGS_SUPPORTS_INFINITE_YAW: u32 = 2048;
    pub const CAP_FLAGS_CAN_POINT_LOCATION_LOCAL: u32 = 65536;
    pub const CAP_FLAGS_CAN_POINT_LOCATION_GLOBAL: u32 = 131072;
}

impl Default for GimbalManagerInformation {
    fn default() -> Self {
        GimbalManagerInformation {
            header: crate::std_msgs::msg::Header::default(),
            cap_flags: 0,
            gimbal_device_id: 0,
            roll_min: 0.0,
            roll_max: 0.0,
            pitch_min: 0.0,
            pitch_max: 0.0,
            yaw_min: 0.0,
            yaw_max: 0.0,
        }
    }
}

impl ros2_client::Message for GimbalManagerInformation {}
