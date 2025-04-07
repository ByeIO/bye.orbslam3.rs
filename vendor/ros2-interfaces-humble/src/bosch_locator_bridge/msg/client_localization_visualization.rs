use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClientLocalizationVisualization {
    pub timestamp: crate::builtin_interfaces::msg::Time,
    pub unique_id: u64,
    pub loc_state: i32,
    pub delay: f64,
}

impl ClientLocalizationVisualization {
    pub const LOC_STATUS_INFO_LOCALIZATION_NOT_RUNNING: i32 = -3;
    pub const LOC_STATUS_NOT_LOCALIZED_STARTUP: i32 = -2;
    pub const LOC_STATUS_NOT_LOCALIZED: i32 = -1;
    pub const LOC_STATUS_LOCALIZED_ODO_ONLY: i32 = 1;
    pub const LOC_STATUS_LOCALIZED: i32 = 2;
    pub const LOC_STATUS_LOCALIZED_INTERNALUSE1: i32 = 3;
    pub const LOC_STATUS_LOCALIZED_INTERNALUSE2: i32 = 4;
}

impl Default for ClientLocalizationVisualization {
    fn default() -> Self {
        ClientLocalizationVisualization {
            timestamp: crate::builtin_interfaces::msg::Time::default(),
            unique_id: 0,
            loc_state: 0,
            delay: 0.0,
        }
    }
}

impl ros2_client::Message for ClientLocalizationVisualization {}
