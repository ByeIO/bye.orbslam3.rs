use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Status {
    pub operating_mode: u16,
    pub area_number: u16,
    pub error_status: bool,
    pub error_code: u16,
    pub lockout_status: bool,
    pub ossd_1: bool,
    pub ossd_2: bool,
    pub warning_1: bool,
    pub warning_2: bool,
    pub ossd_3: bool,
    pub ossd_4: bool,
    pub distance: u16,
    pub angle: f32,
}

impl Status {
    pub const NORMAL: u16 = 0;
    pub const SETTING: u16 = 1;
}

impl Default for Status {
    fn default() -> Self {
        Status {
            operating_mode: 0,
            area_number: 0,
            error_status: false,
            error_code: 0,
            lockout_status: false,
            ossd_1: false,
            ossd_2: false,
            warning_1: false,
            warning_2: false,
            ossd_3: false,
            ossd_4: false,
            distance: 0,
            angle: 0.0,
        }
    }
}

impl ros2_client::Message for Status {}
