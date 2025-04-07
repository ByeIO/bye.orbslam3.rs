use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EsfMEAS {
    pub time_tag: u32,
    pub flags: u16,
    pub id: u16,
    pub data: Vec<u32>,
    pub calib_t_tag: Vec<u32>,
}

impl EsfMEAS {
    pub const CLASS_ID: u8 = 16;
    pub const MESSAGE_ID: u8 = 2;
    pub const FLAGS_TIME_MARK_SENT_MASK: u16 = 3;
    pub const TIME_MARK_NONE: u16 = 0;
    pub const TIME_MARK_EXT0: u16 = 1;
    pub const TIME_MARK_EXT: u16 = 2;
    pub const FLAGS_TIME_MARK_EDGE: u16 = 4;
    pub const FLAGS_CALIB_T_TAG_VALID: u16 = 8;
    pub const DATA_FIELD_MASK: u32 = 16777215;
    pub const DATA_TYPE_MASK: u32 = 1056964608;
    pub const DATA_TYPE_SHIFT: u32 = 24;
    pub const DATA_TYPE_NONE: u32 = 0;
    pub const DATA_TYPE_Z_AXIS_GYRO: u32 = 5;
    pub const DATA_TYPE_WHEEL_TICKS_FRONT_LEFT: u32 = 6;
    pub const DATA_TYPE_WHEEL_TICKS_FRONT_RIGHT: u32 = 7;
    pub const DATA_TYPE_WHEEL_TICKS_REAR_LEFT: u32 = 8;
    pub const DATA_TYPE_WHEEL_TICKS_REAR_RIGHT: u32 = 9;
    pub const DATA_TYPE_SINGLE_TICK: u32 = 10;
    pub const DATA_TYPE_SPEED: u32 = 11;
    pub const DATA_TYPE_GYRO_TEMPERATURE: u32 = 12;
    pub const DATA_TYPE_GYRO_ANG_RATE_Y: u32 = 13;
    pub const DATA_TYPE_GYRO_ANG_RATE_X: u32 = 14;
    pub const DATA_TYPE_ACCELEROMETER_X: u32 = 16;
    pub const DATA_TYPE_ACCELEROMETER_Y: u32 = 17;
    pub const DATA_TYPE_ACCELEROMETER_Z: u32 = 18;
}

impl Default for EsfMEAS {
    fn default() -> Self {
        EsfMEAS {
            time_tag: 0,
            flags: 0,
            id: 0,
            data: Vec::new(),
            calib_t_tag: Vec::new(),
        }
    }
}

impl ros2_client::Message for EsfMEAS {}
