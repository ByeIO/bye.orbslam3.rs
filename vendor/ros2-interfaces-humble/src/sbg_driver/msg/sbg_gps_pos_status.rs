use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SbgGpsPosStatus {
    pub status: u8,
    #[serde(rename = "type")]    pub type_: u8,
    pub ifm: u8,
    pub spoofing: u8,
    pub osnma: u8,
    pub gps_l1_used: bool,
    pub gps_l2_used: bool,
    pub gps_l5_used: bool,
    pub glo_l1_used: bool,
    pub glo_l2_used: bool,
    pub glo_l3_used: bool,
    pub gal_e1_used: bool,
    pub gal_e5a_used: bool,
    pub gal_e5b_used: bool,
    pub gal_e5alt_used: bool,
    pub gal_e6_used: bool,
    pub bds_b1_used: bool,
    pub bds_b2_used: bool,
    pub bds_b3_used: bool,
    pub qzss_l1_used: bool,
    pub qzss_l2_used: bool,
    pub qzss_l5_used: bool,
}

impl Default for SbgGpsPosStatus {
    fn default() -> Self {
        SbgGpsPosStatus {
            status: 0,
            type_: 0,
            ifm: 0,
            spoofing: 0,
            osnma: 0,
            gps_l1_used: false,
            gps_l2_used: false,
            gps_l5_used: false,
            glo_l1_used: false,
            glo_l2_used: false,
            glo_l3_used: false,
            gal_e1_used: false,
            gal_e5a_used: false,
            gal_e5b_used: false,
            gal_e5alt_used: false,
            gal_e6_used: false,
            bds_b1_used: false,
            bds_b2_used: false,
            bds_b3_used: false,
            qzss_l1_used: false,
            qzss_l2_used: false,
            qzss_l5_used: false,
        }
    }
}

impl ros2_client::Message for SbgGpsPosStatus {}
