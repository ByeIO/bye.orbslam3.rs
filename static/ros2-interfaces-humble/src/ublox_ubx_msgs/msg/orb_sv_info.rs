use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrbSVInfo {
    pub gnss_id: u8,
    pub sv_id: u8,
    pub sv_flag: crate::ublox_ubx_msgs::msg::OrbSVFlag,
    pub eph: crate::ublox_ubx_msgs::msg::OrbEphInfo,
    pub alm: crate::ublox_ubx_msgs::msg::OrbAlmInfo,
    pub other_orb: crate::ublox_ubx_msgs::msg::OtherOrbInfo,
}

impl Default for OrbSVInfo {
    fn default() -> Self {
        OrbSVInfo {
            gnss_id: 0,
            sv_id: 0,
            sv_flag: crate::ublox_ubx_msgs::msg::OrbSVFlag::default(),
            eph: crate::ublox_ubx_msgs::msg::OrbEphInfo::default(),
            alm: crate::ublox_ubx_msgs::msg::OrbAlmInfo::default(),
            other_orb: crate::ublox_ubx_msgs::msg::OtherOrbInfo::default(),
        }
    }
}

impl ros2_client::Message for OrbSVInfo {}
