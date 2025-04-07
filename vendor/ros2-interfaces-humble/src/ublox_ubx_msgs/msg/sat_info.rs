use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SatInfo {
    pub gnss_id: u8,
    pub sv_id: u8,
    pub cno: u8,
    pub elev: i8,
    pub azim: i16,
    pub pr_res: i16,
    pub flags: crate::ublox_ubx_msgs::msg::SatFlags,
}

impl Default for SatInfo {
    fn default() -> Self {
        SatInfo {
            gnss_id: 0,
            sv_id: 0,
            cno: 0,
            elev: 0,
            azim: 0,
            pr_res: 0,
            flags: crate::ublox_ubx_msgs::msg::SatFlags::default(),
        }
    }
}

impl ros2_client::Message for SatInfo {}
