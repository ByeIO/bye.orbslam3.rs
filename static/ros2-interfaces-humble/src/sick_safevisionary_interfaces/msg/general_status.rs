use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeneralStatus {
    pub run_mode_active: u8,
    pub device_error: u8,
    pub application_error: u8,
    pub sleep_mode: u8,
    pub wait_for_input: u8,
    pub wait_for_cluster: u8,
    pub contamination_warning: u8,
    pub contamination_error: u8,
    pub dead_zone_detection: u8,
    pub temperature_warning: u8,
}

impl Default for GeneralStatus {
    fn default() -> Self {
        GeneralStatus {
            run_mode_active: 0,
            device_error: 0,
            application_error: 0,
            sleep_mode: 0,
            wait_for_input: 0,
            wait_for_cluster: 0,
            contamination_warning: 0,
            contamination_error: 0,
            dead_zone_detection: 0,
            temperature_warning: 0,
        }
    }
}

impl ros2_client::Message for GeneralStatus {}
