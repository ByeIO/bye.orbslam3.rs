use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HmsInfoMsg {
    pub error_code: u32,
    pub component_index: u8,
    pub error_level: u8,
    pub ground_info: ::std::string::String,
    pub fly_info: ::std::string::String,
}

impl HmsInfoMsg {
    pub const MIN_HMS_ERROR_LEVEL: u8 = 0;
    pub const MID_HMS_ERROR_LEVEL: u8 = 3;
    pub const MAX_HMS_ERROR_LEVEL: u8 = 6;
}

impl Default for HmsInfoMsg {
    fn default() -> Self {
        HmsInfoMsg {
            error_code: 0,
            component_index: 0,
            error_level: 0,
            ground_info: ::std::string::String::new(),
            fly_info: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for HmsInfoMsg {}
