use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EsrVehicle4 {
    pub header: crate::std_msgs::msg::Header,
    pub fac_align_cmd_1: bool,
    pub fac_align_cmd_2: bool,
    pub fac_align_max_nt: u8,
    pub fac_align_samp_req: u8,
    pub fac_tgt_mtg_offset: i8,
    pub fac_tgt_mtg_space_hor: i8,
    pub fac_tgt_mtg_space_ver: i8,
    pub fac_tgt_range_1: f32,
    pub fac_tgt_range_r2m: f32,
    pub fac_tgt_range_m2t: f32,
}

impl Default for EsrVehicle4 {
    fn default() -> Self {
        EsrVehicle4 {
            header: crate::std_msgs::msg::Header::default(),
            fac_align_cmd_1: false,
            fac_align_cmd_2: false,
            fac_align_max_nt: 0,
            fac_align_samp_req: 0,
            fac_tgt_mtg_offset: 0,
            fac_tgt_mtg_space_hor: 0,
            fac_tgt_mtg_space_ver: 0,
            fac_tgt_range_1: 0.0,
            fac_tgt_range_r2m: 0.0,
            fac_tgt_range_m2t: 0.0,
        }
    }
}

impl ros2_client::Message for EsrVehicle4 {}
