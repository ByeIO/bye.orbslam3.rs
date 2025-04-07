use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DisplayMode {
    pub header: crate::std_msgs::msg::Header,
    pub display_mode: u8,
}

impl DisplayMode {
    pub const DISPLAY_MODE_MANUAL_CTRL: u8 = 0;
    pub const DISPLAY_MODE_ATTITUDE: u8 = 1;
    pub const DISPLAY_MODE_RESERVED_2: u8 = 2;
    pub const DISPLAY_MODE_RESERVED_3: u8 = 3;
    pub const DISPLAY_MODE_RESERVED_4: u8 = 4;
    pub const DISPLAY_MODE_RESERVED_5: u8 = 5;
    pub const DISPLAY_MODE_P_GPS: u8 = 6;
    pub const DISPLAY_MODE_RESERVED_7: u8 = 7;
    pub const DISPLAY_MODE_RESERVED_8: u8 = 8;
    pub const DISPLAY_MODE_HOTPOINT_MODE: u8 = 9;
    pub const DISPLAY_MODE_ASSISTED_TAKEOFF: u8 = 10;
    pub const DISPLAY_MODE_AUTO_TAKEOFF: u8 = 11;
    pub const DISPLAY_MODE_AUTO_LANDING: u8 = 12;
    pub const DISPLAY_MODE_RESERVED_13: u8 = 13;
    pub const DISPLAY_MODE_RESERVED_14: u8 = 14;
    pub const DISPLAY_MODE_NAVI_GO_HOME: u8 = 15;
    pub const DISPLAY_MODE_RESERVED_16: u8 = 16;
    pub const DISPLAY_MODE_NAVI_SDK_CTRL: u8 = 17;
    pub const DISPLAY_MODE_RESERVED_18: u8 = 18;
    pub const DISPLAY_MODE_RESERVED_19: u8 = 19;
    pub const DISPLAY_MODE_RESERVED_20: u8 = 20;
    pub const DISPLAY_MODE_RESERVED_21: u8 = 21;
    pub const DISPLAY_MODE_RESERVED_22: u8 = 22;
    pub const DISPLAY_MODE_RESERVED_23: u8 = 23;
    pub const DISPLAY_MODE_RESERVED_24: u8 = 24;
    pub const DISPLAY_MODE_RESERVED_25: u8 = 25;
    pub const DISPLAY_MODE_RESERVED_26: u8 = 26;
    pub const DISPLAY_MODE_RESERVED_27: u8 = 27;
    pub const DISPLAY_MODE_RESERVED_28: u8 = 28;
    pub const DISPLAY_MODE_RESERVED_29: u8 = 29;
    pub const DISPLAY_MODE_RESERVED_30: u8 = 30;
    pub const DISPLAY_MODE_RESERVED_31: u8 = 31;
    pub const DISPLAY_MODE_RESERVED_32: u8 = 32;
    pub const DISPLAY_MODE_FORCE_AUTO_LANDING: u8 = 33;
    pub const DISPLAY_MODE_RESERVED_34: u8 = 34;
    pub const DISPLAY_MODE_RESERVED_35: u8 = 35;
    pub const DISPLAY_MODE_RESERVED_36: u8 = 36;
    pub const DISPLAY_MODE_RESERVED_37: u8 = 37;
    pub const DISPLAY_MODE_RESERVED_38: u8 = 38;
    pub const DISPLAY_MODE_RESERVED_39: u8 = 39;
    pub const DISPLAY_MODE_SEARCH_MODE: u8 = 40;
    pub const DISPLAY_MODE_ENGINE_START: u8 = 41;
    pub const DISPLAY_MODE_RESERVED_42: u8 = 42;
    pub const DISPLAY_MODE_RESERVED_43: u8 = 42;
}

impl Default for DisplayMode {
    fn default() -> Self {
        DisplayMode {
            header: crate::std_msgs::msg::Header::default(),
            display_mode: 0,
        }
    }
}

impl ros2_client::Message for DisplayMode {}
