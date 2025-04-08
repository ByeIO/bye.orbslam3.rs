use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UBXNavRelPosNED {
    pub header: crate::std_msgs::msg::Header,
    pub version: u8,
    pub ref_station_id: u16,
    pub itow: u32,
    pub rel_pos_n: i32,
    pub rel_pos_e: i32,
    pub rel_pos_d: i32,
    pub rel_pos_length: i32,
    pub rel_pos_heading: i32,
    pub rel_pos_hp_n: i8,
    pub rel_pos_hp_e: i8,
    pub rel_pos_hp_d: i8,
    pub rel_pos_hp_length: i8,
    pub acc_n: u32,
    pub acc_e: u32,
    pub acc_d: u32,
    pub acc_length: u32,
    pub acc_heading: u32,
    pub gnss_fix_ok: bool,
    pub diff_soln: bool,
    pub rel_pos_valid: bool,
    pub carr_soln: crate::ublox_ubx_msgs::msg::CarrSoln,
    pub is_moving: bool,
    pub ref_pos_miss: bool,
    pub ref_obs_miss: bool,
    pub rel_pos_heading_valid: bool,
    pub rel_pos_normalized: bool,
}

impl Default for UBXNavRelPosNED {
    fn default() -> Self {
        UBXNavRelPosNED {
            header: crate::std_msgs::msg::Header::default(),
            version: 0,
            ref_station_id: 0,
            itow: 0,
            rel_pos_n: 0,
            rel_pos_e: 0,
            rel_pos_d: 0,
            rel_pos_length: 0,
            rel_pos_heading: 0,
            rel_pos_hp_n: 0,
            rel_pos_hp_e: 0,
            rel_pos_hp_d: 0,
            rel_pos_hp_length: 0,
            acc_n: 0,
            acc_e: 0,
            acc_d: 0,
            acc_length: 0,
            acc_heading: 0,
            gnss_fix_ok: false,
            diff_soln: false,
            rel_pos_valid: false,
            carr_soln: crate::ublox_ubx_msgs::msg::CarrSoln::default(),
            is_moving: false,
            ref_pos_miss: false,
            ref_obs_miss: false,
            rel_pos_heading_valid: false,
            rel_pos_normalized: false,
        }
    }
}

impl ros2_client::Message for UBXNavRelPosNED {}
