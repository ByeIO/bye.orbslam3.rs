use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GimbalManagerStatus {
    pub header: crate::std_msgs::msg::Header,
    pub flags: u32,
    pub gimbal_device_id: u8,
    pub sysid_primary: u8,
    pub compid_primary: u8,
    pub sysid_secondary: u8,
    pub compid_secondary: u8,
}

impl GimbalManagerStatus {
    pub const GIMBAL_MANAGER_FLAGS_RETRACT: u32 = 1;
    pub const GIMBAL_MANAGER_FLAGS_NEUTRAL: u32 = 2;
    pub const GIMBAL_MANAGER_FLAGS_ROLL_LOCK: u32 = 4;
    pub const GIMBAL_MANAGER_FLAGS_PITCH_LOCK: u32 = 8;
    pub const GIMBAL_MANAGER_FLAGS_YAW_LOCK: u32 = 16;
}

impl Default for GimbalManagerStatus {
    fn default() -> Self {
        GimbalManagerStatus {
            header: crate::std_msgs::msg::Header::default(),
            flags: 0,
            gimbal_device_id: 0,
            sysid_primary: 0,
            compid_primary: 0,
            sysid_secondary: 0,
            compid_secondary: 0,
        }
    }
}

impl ros2_client::Message for GimbalManagerStatus {}
