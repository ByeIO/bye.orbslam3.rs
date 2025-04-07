use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UBXNavTimeUTC {
    pub header: crate::std_msgs::msg::Header,
    pub itow: u32,
    pub t_acc: u32,
    pub nano: i32,
    pub year: i16,
    pub month: i8,
    pub day: i8,
    pub hour: i8,
    pub min: i8,
    pub sec: i8,
    pub valid_tow: bool,
    pub valid_wkn: bool,
    pub valid_utc: bool,
    pub utc_std: crate::ublox_ubx_msgs::msg::UtcStd,
}

impl Default for UBXNavTimeUTC {
    fn default() -> Self {
        UBXNavTimeUTC {
            header: crate::std_msgs::msg::Header::default(),
            itow: 0,
            t_acc: 0,
            nano: 0,
            year: 0,
            month: 0,
            day: 0,
            hour: 0,
            min: 0,
            sec: 0,
            valid_tow: false,
            valid_wkn: false,
            valid_utc: false,
            utc_std: crate::ublox_ubx_msgs::msg::UtcStd::default(),
        }
    }
}

impl ros2_client::Message for UBXNavTimeUTC {}
