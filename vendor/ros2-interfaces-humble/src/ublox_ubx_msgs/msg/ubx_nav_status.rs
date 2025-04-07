use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UBXNavStatus {
    pub header: crate::std_msgs::msg::Header,
    pub itow: u32,
    pub gps_fix: crate::ublox_ubx_msgs::msg::GpsFix,
    pub gps_fix_ok: bool,
    pub diff_soln: bool,
    pub wkn_set: bool,
    pub tow_set: bool,
    pub diff_corr: bool,
    pub carr_soln_valid: bool,
    pub map_matching: crate::ublox_ubx_msgs::msg::MapMatching,
    pub psm: crate::ublox_ubx_msgs::msg::PSMStatus,
    pub spoof_det: crate::ublox_ubx_msgs::msg::SpoofDet,
    pub carr_soln: crate::ublox_ubx_msgs::msg::CarrSoln,
    pub ttff: u32,
    pub msss: u32,
}

impl Default for UBXNavStatus {
    fn default() -> Self {
        UBXNavStatus {
            header: crate::std_msgs::msg::Header::default(),
            itow: 0,
            gps_fix: crate::ublox_ubx_msgs::msg::GpsFix::default(),
            gps_fix_ok: false,
            diff_soln: false,
            wkn_set: false,
            tow_set: false,
            diff_corr: false,
            carr_soln_valid: false,
            map_matching: crate::ublox_ubx_msgs::msg::MapMatching::default(),
            psm: crate::ublox_ubx_msgs::msg::PSMStatus::default(),
            spoof_det: crate::ublox_ubx_msgs::msg::SpoofDet::default(),
            carr_soln: crate::ublox_ubx_msgs::msg::CarrSoln::default(),
            ttff: 0,
            msss: 0,
        }
    }
}

impl ros2_client::Message for UBXNavStatus {}
