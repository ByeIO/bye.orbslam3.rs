use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SrrStatus2 {
    pub header: crate::std_msgs::msg::Header,
    pub can_tx_alignment_status: u8,
    pub can_tx_comm_error: bool,
    pub can_tx_steering_angle_sign: bool,
    pub can_tx_yaw_rate_bias: f32,
    pub can_tx_veh_spd_comp_factor: f32,
    pub can_tx_sw_version_dsp: u16,
    pub can_tx_temperature: i16,
    pub can_tx_range_perf_error: bool,
    pub can_tx_overheat_error: bool,
    pub can_tx_internal_error: bool,
    pub can_tx_xcvr_operational: bool,
    pub can_tx_steering_angle: u16,
    pub can_tx_rolling_count_2: u8,
}

impl SrrStatus2 {
    pub const CAN_TX_ALIGNMENT_STATUS_UNKNOWN: u8 = 0;
    pub const CAN_TX_ALIGNMENT_STATUS_CONVERGED: u8 = 1;
    pub const CAN_TX_ALIGNMENT_STATUS_FAILED: u8 = 2;
    pub const CAN_TX_ALIGNMENT_STATUS_RESERVED: u8 = 3;
}

impl Default for SrrStatus2 {
    fn default() -> Self {
        SrrStatus2 {
            header: crate::std_msgs::msg::Header::default(),
            can_tx_alignment_status: 0,
            can_tx_comm_error: false,
            can_tx_steering_angle_sign: false,
            can_tx_yaw_rate_bias: 0.0,
            can_tx_veh_spd_comp_factor: 0.0,
            can_tx_sw_version_dsp: 0,
            can_tx_temperature: 0,
            can_tx_range_perf_error: false,
            can_tx_overheat_error: false,
            can_tx_internal_error: false,
            can_tx_xcvr_operational: false,
            can_tx_steering_angle: 0,
            can_tx_rolling_count_2: 0,
        }
    }
}

impl ros2_client::Message for SrrStatus2 {}
