use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SbgAirDataStatus {
    pub is_delay_time: bool,
    pub pressure_valid: bool,
    pub altitude_valid: bool,
    pub pressure_diff_valid: bool,
    pub air_speed_valid: bool,
    pub air_temperature_valid: bool,
}

impl Default for SbgAirDataStatus {
    fn default() -> Self {
        SbgAirDataStatus {
            is_delay_time: false,
            pressure_valid: false,
            altitude_valid: false,
            pressure_diff_valid: false,
            air_speed_valid: false,
            air_temperature_valid: false,
        }
    }
}

impl ros2_client::Message for SbgAirDataStatus {}
