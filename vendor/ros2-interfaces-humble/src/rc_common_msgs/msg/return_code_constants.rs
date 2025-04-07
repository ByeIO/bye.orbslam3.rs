use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReturnCodeConstants {

}

impl ReturnCodeConstants {
    pub const SUCCESS: i16 = 0;
    pub const INVALID_ARGUMENT: i16 = -1;
    pub const INTERNAL_ERROR: i16 = -2;
    pub const INTERNAL_TIMEOUT: i16 = -3;
    pub const SENSOR_TIMEOUT: i16 = -4;
    pub const IO_ERROR: i16 = -7;
    pub const NOT_APPLICABLE: i16 = -8;
    pub const INVALID_LICENSE: i16 = -9;
    pub const CAPACITY_EXCEEDED: i16 = -10;
    pub const TIMEOUT_REACHED: i16 = 3;
    pub const CAPACITY_REACHED: i16 = 10;
    pub const OVERWRITTEN: i16 = 11;
    pub const HINTS: i16 = 999;
}

impl Default for ReturnCodeConstants {
    fn default() -> Self {
        ReturnCodeConstants {

        }
    }
}

impl ros2_client::Message for ReturnCodeConstants {}
