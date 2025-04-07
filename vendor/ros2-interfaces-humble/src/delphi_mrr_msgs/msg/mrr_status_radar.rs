use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MrrStatusRadar {
    pub header: crate::std_msgs::msg::Header,
    pub can_interference_type: u8,
    pub can_recommend_unconverge: bool,
    pub can_blockage_sidelobe_filter_val: u8,
    pub can_radar_align_incomplete: bool,
    pub can_blockage_sidelobe: bool,
    pub can_blockage_mnr: bool,
    pub can_radar_ext_cond_nok: bool,
    pub can_radar_align_out_range: bool,
    pub can_radar_align_not_start: bool,
    pub can_radar_overheat_error: bool,
    pub can_radar_not_op: bool,
    pub can_xcvr_operational: bool,
}

impl Default for MrrStatusRadar {
    fn default() -> Self {
        MrrStatusRadar {
            header: crate::std_msgs::msg::Header::default(),
            can_interference_type: 0,
            can_recommend_unconverge: false,
            can_blockage_sidelobe_filter_val: 0,
            can_radar_align_incomplete: false,
            can_blockage_sidelobe: false,
            can_blockage_mnr: false,
            can_radar_ext_cond_nok: false,
            can_radar_align_out_range: false,
            can_radar_align_not_start: false,
            can_radar_overheat_error: false,
            can_radar_not_op: false,
            can_xcvr_operational: false,
        }
    }
}

impl ros2_client::Message for MrrStatusRadar {}
