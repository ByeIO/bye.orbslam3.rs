use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AwsDisplay {
    pub header: crate::std_msgs::msg::Header,
    pub suppress_sound: bool,
    pub night_time: bool,
    pub dusk_time: bool,
    pub sound_type: u8,
    pub headway_valid: bool,
    pub headway_measurement: f32,
    pub lanes_on: bool,
    pub left_ldw_on: bool,
    pub right_ldw_on: bool,
    pub fcw_on: bool,
    pub left_crossing: bool,
    pub right_crossing: bool,
    pub maintenance: bool,
    pub failsafe: bool,
    pub ped_fcw: bool,
    pub ped_in_dz: bool,
    pub headway_warning_level: u8,
}

impl AwsDisplay {
    pub const SOUND_SILENT: u8 = 0;
    pub const SOUND_LDWL: u8 = 1;
    pub const SOUND_LDWR: u8 = 2;
    pub const SOUND_FAR_HW: u8 = 3;
    pub const SOUND_NEAR_HW: u8 = 4;
    pub const SOUND_SOFT_FCW: u8 = 5;
    pub const SOUND_HARD_FCW: u8 = 6;
    pub const SOUND_RESERVED: u8 = 7;
    pub const HEADWAY_LEVEL_OFF: u8 = 0;
    pub const HEADWAY_LEVEL_GREEN: u8 = 1;
    pub const HEADWAY_LEVEL_ORANGE: u8 = 2;
    pub const HEADWAY_LEVEL_RED: u8 = 3;
}

impl Default for AwsDisplay {
    fn default() -> Self {
        AwsDisplay {
            header: crate::std_msgs::msg::Header::default(),
            suppress_sound: false,
            night_time: false,
            dusk_time: false,
            sound_type: 0,
            headway_valid: false,
            headway_measurement: 0.0,
            lanes_on: false,
            left_ldw_on: false,
            right_ldw_on: false,
            fcw_on: false,
            left_crossing: false,
            right_crossing: false,
            maintenance: false,
            failsafe: false,
            ped_fcw: false,
            ped_in_dz: false,
            headway_warning_level: 0,
        }
    }
}

impl ros2_client::Message for AwsDisplay {}
