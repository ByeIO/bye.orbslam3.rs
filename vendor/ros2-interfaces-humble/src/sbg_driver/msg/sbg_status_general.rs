use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SbgStatusGeneral {
    pub main_power: bool,
    pub imu_power: bool,
    pub gps_power: bool,
    pub settings: bool,
    pub temperature: bool,
    pub datalogger: bool,
    pub cpu: bool,
}

impl Default for SbgStatusGeneral {
    fn default() -> Self {
        SbgStatusGeneral {
            main_power: false,
            imu_power: false,
            gps_power: false,
            settings: false,
            temperature: false,
            datalogger: false,
            cpu: false,
        }
    }
}

impl ros2_client::Message for SbgStatusGeneral {}
