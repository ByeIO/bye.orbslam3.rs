use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GimbalDeviceInformation {
    pub header: crate::std_msgs::msg::Header,
    pub vendor_name: ::std::string::String,
    pub model_name: ::std::string::String,
    pub custom_name: ::std::string::String,
    pub firmware_version: u32,
    pub hardware_version: u32,
    pub uid: u64,
    pub cap_flags: u32,
    pub custom_cap_flags: u16,
    pub roll_min: f32,
    pub roll_max: f32,
    pub pitch_min: f32,
    pub pitch_max: f32,
    pub yaw_min: f32,
    pub yaw_max: f32,
}

impl GimbalDeviceInformation {
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
}

impl Default for GimbalDeviceInformation {
    fn default() -> Self {
        GimbalDeviceInformation {
            header: crate::std_msgs::msg::Header::default(),
            vendor_name: ::std::string::String::new(),
            model_name: ::std::string::String::new(),
            custom_name: ::std::string::String::new(),
            firmware_version: 0,
            hardware_version: 0,
            uid: 0,
            cap_flags: 0,
            custom_cap_flags: 0,
            roll_min: 0.0,
            roll_max: 0.0,
            pitch_min: 0.0,
            pitch_max: 0.0,
            yaw_min: 0.0,
            yaw_max: 0.0,
        }
    }
}

impl ros2_client::Message for GimbalDeviceInformation {}
