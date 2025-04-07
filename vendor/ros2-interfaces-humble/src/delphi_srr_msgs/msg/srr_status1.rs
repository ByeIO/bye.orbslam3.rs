use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SrrStatus1 {
    pub header: crate::std_msgs::msg::Header,
    pub can_tx_look_type: bool,
    pub can_tx_dsp_timestamp: u32,
    pub can_tx_yaw_rate_calc: f32,
    pub can_tx_vehicle_speed_calc: f32,
    pub can_tx_scan_index: u16,
    pub can_tx_curvature: f32,
}

impl Default for SrrStatus1 {
    fn default() -> Self {
        SrrStatus1 {
            header: crate::std_msgs::msg::Header::default(),
            can_tx_look_type: false,
            can_tx_dsp_timestamp: 0,
            can_tx_yaw_rate_calc: 0.0,
            can_tx_vehicle_speed_calc: 0.0,
            can_tx_scan_index: 0,
            can_tx_curvature: 0.0,
        }
    }
}

impl ros2_client::Message for SrrStatus1 {}
