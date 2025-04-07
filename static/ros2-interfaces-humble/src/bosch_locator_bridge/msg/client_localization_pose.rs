use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClientLocalizationPose {
    pub age: crate::builtin_interfaces::msg::Duration,
    pub timestamp: crate::builtin_interfaces::msg::Time,
    pub unique_id: u64,
    pub state: i32,
    pub epoch: u64,
    pub error_flags: u64,
    pub info_flags: u64,
}

impl ClientLocalizationPose {
    pub const LOC_STATUS_INFO_LOCALIZATION_NOT_RUNNING: i32 = -3;
    pub const LOC_STATUS_NOT_LOCALIZED_STARTUP: i32 = -2;
    pub const LOC_STATUS_NOT_LOCALIZED: i32 = -1;
    pub const LOC_STATUS_LOCALIZED_ODO_ONLY: i32 = 1;
    pub const LOC_STATUS_LOCALIZED: i32 = 2;
    pub const LOC_STATUS_LOCALIZED_INTERNALUSE1: i32 = 3;
    pub const LOC_STATUS_LOCALIZED_INTERNALUSE2: i32 = 4;
}

impl Default for ClientLocalizationPose {
    fn default() -> Self {
        ClientLocalizationPose {
            age: crate::builtin_interfaces::msg::Duration::default(),
            timestamp: crate::builtin_interfaces::msg::Time::default(),
            unique_id: 0,
            state: 0,
            epoch: 0,
            error_flags: 0,
            info_flags: 0,
        }
    }
}

impl ros2_client::Message for ClientLocalizationPose {}
