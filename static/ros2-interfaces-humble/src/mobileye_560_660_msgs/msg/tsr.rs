use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tsr {
    pub header: crate::std_msgs::msg::Header,
    pub vision_only_sign_type: u8,
    pub vision_only_supplementary_sign_type: u8,
    pub sign_position_x: f32,
    pub sign_position_y: f32,
    pub sign_position_z: f32,
    pub filter_type: u8,
}

impl Tsr {
    pub const SIGN_TYPE_REGULAR_10: u8 = 0;
    pub const SIGN_TYPE_REGULAR_20: u8 = 1;
    pub const SIGN_TYPE_REGULAR_30: u8 = 2;
    pub const SIGN_TYPE_REGULAR_40: u8 = 3;
    pub const SIGN_TYPE_REGULAR_50: u8 = 4;
    pub const SIGN_TYPE_REGULAR_60: u8 = 5;
    pub const SIGN_TYPE_REGULAR_70: u8 = 6;
    pub const SIGN_TYPE_REGULAR_80: u8 = 7;
    pub const SIGN_TYPE_REGULAR_90: u8 = 8;
    pub const SIGN_TYPE_REGULAR_100: u8 = 9;
    pub const SIGN_TYPE_REGULAR_110: u8 = 10;
    pub const SIGN_TYPE_REGULAR_120: u8 = 11;
    pub const SIGN_TYPE_REGULAR_130: u8 = 12;
    pub const SIGN_TYPE_REGULAR_140: u8 = 13;
    pub const SIGN_TYPE_REGULAR_END_RESTRICTION_OF_NUMBER: u8 = 20;
    pub const SIGN_TYPE_ELECTRONIC_10: u8 = 28;
    pub const SIGN_TYPE_ELECTRONIC_20: u8 = 29;
    pub const SIGN_TYPE_ELECTRONIC_30: u8 = 30;
    pub const SIGN_TYPE_ELECTRONIC_40: u8 = 31;
    pub const SIGN_TYPE_ELECTRONIC_50: u8 = 32;
    pub const SIGN_TYPE_ELECTRONIC_60: u8 = 33;
    pub const SIGN_TYPE_ELECTRONIC_70: u8 = 34;
    pub const SIGN_TYPE_ELECTRONIC_80: u8 = 35;
    pub const SIGN_TYPE_ELECTRONIC_90: u8 = 36;
    pub const SIGN_TYPE_ELECTRONIC_100: u8 = 37;
    pub const SIGN_TYPE_ELECTRONIC_110: u8 = 38;
    pub const SIGN_TYPE_ELECTRONIC_120: u8 = 39;
    pub const SIGN_TYPE_ELECTRONIC_130: u8 = 40;
    pub const SIGN_TYPE_ELECTRONIC_140: u8 = 41;
    pub const SIGN_TYPE_ELECTRONIC_END_RESTRICTION_OF_NUMBER: u8 = 50;
    pub const SIGN_TYPE_REGULAR_GENERAL_END_ALL_RESTRICTION: u8 = 64;
    pub const SIGN_TYPE_ELECTRONIC_GENERAL_END_ALL_RESTRICTION: u8 = 65;
    pub const SIGN_TYPE_REGULAR_5: u8 = 100;
    pub const SIGN_TYPE_REGULAR_15: u8 = 101;
    pub const SIGN_TYPE_REGULAR_25: u8 = 102;
    pub const SIGN_TYPE_REGULAR_35: u8 = 103;
    pub const SIGN_TYPE_REGULAR_45: u8 = 104;
    pub const SIGN_TYPE_REGULAR_55: u8 = 105;
    pub const SIGN_TYPE_REGULAR_65: u8 = 106;
    pub const SIGN_TYPE_REGULAR_75: u8 = 107;
    pub const SIGN_TYPE_REGULAR_85: u8 = 108;
    pub const SIGN_TYPE_REGULAR_95: u8 = 109;
    pub const SIGN_TYPE_REGULAR_105: u8 = 110;
    pub const SIGN_TYPE_REGULAR_115: u8 = 111;
    pub const SIGN_TYPE_REGULAR_125: u8 = 112;
    pub const SIGN_TYPE_REGULAR_135: u8 = 113;
    pub const SIGN_TYPE_REGULAR_145: u8 = 114;
    pub const SIGN_TYPE_ELECTRONIC_5: u8 = 115;
    pub const SIGN_TYPE_ELECTRONIC_15: u8 = 116;
    pub const SIGN_TYPE_ELECTRONIC_25: u8 = 117;
    pub const SIGN_TYPE_ELECTRONIC_35: u8 = 118;
    pub const SIGN_TYPE_ELECTRONIC_45: u8 = 119;
    pub const SIGN_TYPE_ELECTRONIC_55: u8 = 120;
    pub const SIGN_TYPE_ELECTRONIC_65: u8 = 121;
    pub const SIGN_TYPE_ELECTRONIC_75: u8 = 122;
    pub const SIGN_TYPE_ELECTRONIC_85: u8 = 123;
    pub const SIGN_TYPE_ELECTRONIC_95: u8 = 124;
    pub const SIGN_TYPE_ELECTRONIC_105: u8 = 125;
    pub const SIGN_TYPE_ELECTRONIC_115: u8 = 126;
    pub const SIGN_TYPE_ELECTRONIC_125: u8 = 127;
    pub const SIGN_TYPE_ELECTRONIC_135: u8 = 128;
    pub const SIGN_TYPE_ELECTRONIC_145: u8 = 129;
    pub const SIGN_TYPE_REGULAR_MOTORWAY_BEGIN: u8 = 171;
    pub const SIGN_TYPE_REGULAR_END_OF_MOTORWAY: u8 = 172;
    pub const SIGN_TYPE_REGULAR_EXPRESSWAY_BEGIN: u8 = 173;
    pub const SIGN_TYPE_REGULAR_END_OF_EXPRESSWAY: u8 = 174;
    pub const SIGN_TYPE_REGULAR_PLAYGROUND_AREA_BEGIN: u8 = 175;
    pub const SIGN_TYPE_REGULAR_END_OF_PLAYGROUND_AREA: u8 = 176;
    pub const SIGN_TYPE_REGULAR_NO_PASSING_START: u8 = 200;
    pub const SIGN_TYPE_REGULAR_END_OF_NO_PASSING: u8 = 201;
    pub const SIGN_TYPE_ELECTRONIC_NO_PASSING_START: u8 = 220;
    pub const SIGN_TYPE_ELECTRONIC_END_OF_NO_PASSING: u8 = 221;
    pub const SIGN_TYPE_NONE_DETECTED: u8 = 254;
    pub const SIGN_TYPE_INVALID: u8 = 255;
    pub const SUPP_SIGN_TYPE_NONE: u8 = 0;
    pub const SUPP_SIGN_TYPE_RAIN: u8 = 1;
    pub const SUPP_SIGN_TYPE_SNOW: u8 = 2;
    pub const SUPP_SIGN_TYPE_TRAILER: u8 = 3;
    pub const SUPP_SIGN_TYPE_TIME: u8 = 4;
    pub const SUPP_SIGN_TYPE_ARROW_LEFT: u8 = 5;
    pub const SUPP_SIGN_TYPE_ARROW_RIGHT: u8 = 6;
    pub const SUPP_SIGN_TYPE_BEND_ARROW_LEFT: u8 = 7;
    pub const SUPP_SIGN_TYPE_BEND_ARROW_RIGHT: u8 = 8;
    pub const SUPP_SIGN_TYPE_TRUCK: u8 = 9;
    pub const SUPP_SIGN_TYPE_DISTANCE_ARROW: u8 = 10;
    pub const SUPP_SIGN_TYPE_WEIGHT: u8 = 11;
    pub const SUPP_SIGN_TYPE_DISTANCE_IN: u8 = 12;
    pub const SUPP_SIGN_TYPE_TRACTOR: u8 = 13;
    pub const SUPP_SIGN_TYPE_SNOW_RAIN: u8 = 14;
    pub const SUPP_SIGN_TYPE_SCHOOL: u8 = 15;
    pub const SUPP_SIGN_TYPE_RAIN_CLOUD: u8 = 16;
    pub const SUPP_SIGN_TYPE_FOG: u8 = 17;
    pub const SUPP_SIGN_TYPE_HAZARDOUS_MATERIALS: u8 = 18;
    pub const SUPP_SIGN_TYPE_NIGHT: u8 = 19;
    pub const SUPP_SIGN_TYPE_GENERIC: u8 = 20;
    pub const SUPP_SIGN_TYPE_RAPPEL: u8 = 21;
    pub const SUPP_SIGN_TYPE_ZONE: u8 = 22;
    pub const SUPP_SIGN_TYPE_INVALID: u8 = 255;
    pub const FILTER_TYPE_NOT_FILTERED: u8 = 0;
    pub const FILTER_TYPE_IRRELEVANT_TO_DRIVER: u8 = 1;
    pub const FILTER_TYPE_ON_VEHICLE: u8 = 2;
    pub const FILTER_TYPE_EMBEDDED: u8 = 3;
}

impl Default for Tsr {
    fn default() -> Self {
        Tsr {
            header: crate::std_msgs::msg::Header::default(),
            vision_only_sign_type: 0,
            vision_only_supplementary_sign_type: 0,
            sign_position_x: 0.0,
            sign_position_y: 0.0,
            sign_position_z: 0.0,
            filter_type: 0,
        }
    }
}

impl ros2_client::Message for Tsr {}
