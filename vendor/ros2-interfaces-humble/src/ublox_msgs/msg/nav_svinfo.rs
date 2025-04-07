use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NavSVINFO {
    pub i_tow: u32,
    pub num_ch: u8,
    pub global_flags: u8,
    pub reserved2: u16,
    pub sv: Vec<crate::ublox_msgs::msg::NavSVINFOSV>,
}

impl NavSVINFO {
    pub const CLASS_ID: u8 = 1;
    pub const MESSAGE_ID: u8 = 48;
    pub const CHIPGEN_ANTARIS: u8 = 0;
    pub const CHIPGEN_UBLOX5: u8 = 1;
    pub const CHIPGEN_UBLOX6: u8 = 2;
    pub const CHIPGEN_UBLOX7: u8 = 3;
    pub const CHIPGEN_UBLOX8: u8 = 4;
}

impl Default for NavSVINFO {
    fn default() -> Self {
        NavSVINFO {
            i_tow: 0,
            num_ch: 0,
            global_flags: 0,
            reserved2: 0,
            sv: Vec::new(),
        }
    }
}

impl ros2_client::Message for NavSVINFO {}
