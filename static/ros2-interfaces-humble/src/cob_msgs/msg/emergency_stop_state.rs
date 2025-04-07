use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmergencyStopState {
    pub emergency_button_stop: bool,
    pub scanner_stop: bool,
    pub monitoring_stop: bool,
    pub user_interaction_stop: bool,
    pub hardware_stop: bool,
    pub bumper_stop: bool,
    pub fall_stop: bool,
    pub charge_stop: bool,
    pub emergency_state: i16,
}

impl EmergencyStopState {
    pub const EMFREE: i16 = 0;
    pub const EMSTOP: i16 = 1;
    pub const EMCONFIRMED: i16 = 2;
}

impl Default for EmergencyStopState {
    fn default() -> Self {
        EmergencyStopState {
            emergency_button_stop: false,
            scanner_stop: false,
            monitoring_stop: false,
            user_interaction_stop: false,
            hardware_stop: false,
            bumper_stop: false,
            fall_stop: false,
            charge_stop: false,
            emergency_state: 0,
        }
    }
}

impl ros2_client::Message for EmergencyStopState {}
