use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GPSRTK {
    pub header: crate::std_msgs::msg::Header,
    pub rtk_receiver_id: u8,
    pub wn: i16,
    pub tow: u32,
    pub rtk_health: u8,
    pub rtk_rate: u8,
    pub nsats: u8,
    pub baseline_a: i32,
    pub baseline_b: i32,
    pub baseline_c: i32,
    pub accuracy: u32,
    pub iar_num_hypotheses: i32,
}

impl Default for GPSRTK {
    fn default() -> Self {
        GPSRTK {
            header: crate::std_msgs::msg::Header::default(),
            rtk_receiver_id: 0,
            wn: 0,
            tow: 0,
            rtk_health: 0,
            rtk_rate: 0,
            nsats: 0,
            baseline_a: 0,
            baseline_b: 0,
            baseline_c: 0,
            accuracy: 0,
            iar_num_hypotheses: 0,
        }
    }
}

impl ros2_client::Message for GPSRTK {}
