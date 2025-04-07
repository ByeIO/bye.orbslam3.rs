use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ESFSensorStatus {
    pub sensor_data_type: u8,
    pub used: bool,
    pub ready: bool,
    pub calib_status: u8,
    pub time_status: u8,
    pub freq: u8,
    pub fault_bad_meas: bool,
    pub fault_bad_ttag: bool,
    pub fault_missing_meas: bool,
    pub fault_noisy_meas: bool,
}

impl ESFSensorStatus {
    pub const CALIB_STATUS_NOT_CALIBRATED: u8 = 0b00;
    pub const CALIB_STATUS_CALIBRATING: u8 = 0b01;
    pub const CALIB_STATUS_CALIBRATED0: u8 = 0b10;
    pub const CALIB_STATUS_CALIBRATED1: u8 = 0b11;
    pub const TIME_STATUS_NO_DATA: u8 = 0b00;
    pub const TIME_STATUS_FIRST_BYTE_USED: u8 = 0b01;
    pub const TIME_STATUS_TTAG_PROVIDED: u8 = 0b11;
}

impl Default for ESFSensorStatus {
    fn default() -> Self {
        ESFSensorStatus {
            sensor_data_type: 0,
            used: false,
            ready: false,
            calib_status: 0,
            time_status: 0,
            freq: 0,
            fault_bad_meas: false,
            fault_bad_ttag: false,
            fault_missing_meas: false,
            fault_noisy_meas: false,
        }
    }
}

impl ros2_client::Message for ESFSensorStatus {}
