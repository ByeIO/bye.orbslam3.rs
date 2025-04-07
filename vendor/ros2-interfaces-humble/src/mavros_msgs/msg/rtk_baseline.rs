use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RTKBaseline {
    pub header: crate::std_msgs::msg::Header,
    pub time_last_baseline_ms: u32,
    pub rtk_receiver_id: u8,
    pub wn: u16,
    pub tow: u32,
    pub rtk_health: u8,
    pub rtk_rate: u8,
    pub nsats: u8,
    pub baseline_coords_type: u8,
    pub baseline_a_mm: i32,
    pub baseline_b_mm: i32,
    pub baseline_c_mm: i32,
    pub accuracy: u32,
    pub iar_num_hypotheses: i32,
}

impl RTKBaseline {
    pub const RTK_BASELINE_COORDINATE_SYSTEM_ECEF: u8 = 0;
    pub const RTK_BASELINE_COORDINATE_SYSTEM_NED: u8 = 1;
}

impl Default for RTKBaseline {
    fn default() -> Self {
        RTKBaseline {
            header: crate::std_msgs::msg::Header::default(),
            time_last_baseline_ms: 0,
            rtk_receiver_id: 0,
            wn: 0,
            tow: 0,
            rtk_health: 0,
            rtk_rate: 0,
            nsats: 0,
            baseline_coords_type: 0,
            baseline_a_mm: 0,
            baseline_b_mm: 0,
            baseline_c_mm: 0,
            accuracy: 0,
            iar_num_hypotheses: 0,
        }
    }
}

impl ros2_client::Message for RTKBaseline {}
