use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClientControlMode {
    pub stamp: crate::builtin_interfaces::msg::Time,
    pub visual_recording_state: u8,
    pub map_state: u8,
    pub localization_state: u8,
    pub recording_state: u8,
    pub alignment_state: u8,
    pub mask_state: u8,
    pub expandmap_state: u8,
}

impl ClientControlMode {
    pub const CLIENT_CONTROL_STATE_INIT: u8 = 0;
    pub const CLIENT_CONTROL_STATE_READY: u8 = 1;
    pub const CLIENT_CONTROL_STATE_RUN: u8 = 2;
    pub const CLIENT_CONTROL_STATE_NOT_AVAILABLE: u8 = 4;
}

impl Default for ClientControlMode {
    fn default() -> Self {
        ClientControlMode {
            stamp: crate::builtin_interfaces::msg::Time::default(),
            visual_recording_state: 0,
            map_state: 0,
            localization_state: 0,
            recording_state: 0,
            alignment_state: 0,
            mask_state: 0,
            expandmap_state: 0,
        }
    }
}

impl ros2_client::Message for ClientControlMode {}
