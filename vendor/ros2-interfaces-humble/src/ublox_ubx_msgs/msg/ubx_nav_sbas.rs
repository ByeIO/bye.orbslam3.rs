use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UBXNavSBAS {
    pub header: crate::std_msgs::msg::Header,
    pub itow: u32,
    pub geo: u8,
    pub mode: u8,
    pub sys: i8,
    pub service: crate::ublox_ubx_msgs::msg::SBASService,
    pub cnt: u8,
    pub status_flags: crate::ublox_ubx_msgs::msg::SBASStatusFlags,
    pub reserved_0: [u8; 2],
    pub sv_data: Vec<crate::ublox_ubx_msgs::msg::SBASSvData>,
}

impl UBXNavSBAS {
    pub const MODE_DISABLED: u8 = 0;
    pub const MODE_ENABLED_INTEGRITY: u8 = 1;
    pub const MODE_ENABLED_TEST: u8 = 3;
    pub const SYS_UNKNOWN: i8 = -1;
    pub const SYS_WAAS: i8 = 0;
    pub const SYS_EGNOS: i8 = 1;
    pub const SYS_MSAS: i8 = 2;
    pub const SYS_GAGAN: i8 = 3;
    pub const SYS_GPS: i8 = 16;
}

impl Default for UBXNavSBAS {
    fn default() -> Self {
        UBXNavSBAS {
            header: crate::std_msgs::msg::Header::default(),
            itow: 0,
            geo: 0,
            mode: 0,
            sys: 0,
            service: crate::ublox_ubx_msgs::msg::SBASService::default(),
            cnt: 0,
            status_flags: crate::ublox_ubx_msgs::msg::SBASStatusFlags::default(),
            reserved_0: [0; 2],
            sv_data: Vec::new(),
        }
    }
}

impl ros2_client::Message for UBXNavSBAS {}
