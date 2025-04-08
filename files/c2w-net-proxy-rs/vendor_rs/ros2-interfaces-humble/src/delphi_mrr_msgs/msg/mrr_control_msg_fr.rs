use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MrrControlMsgFR {
    pub header: crate::std_msgs::msg::Header,
    pub can_sensitivity_profile_select: u8,
    pub can_stop_frequency_frml: u16,
    pub can_stop_frequency_frll: u16,
    pub can_prp_factor_frml: f32,
    pub can_prp_factor_frll: f32,
    pub can_desired_sweep_bw_frml: u8,
    pub can_desired_sweep_bw_frll: u8,
}

impl Default for MrrControlMsgFR {
    fn default() -> Self {
        MrrControlMsgFR {
            header: crate::std_msgs::msg::Header::default(),
            can_sensitivity_profile_select: 0,
            can_stop_frequency_frml: 0,
            can_stop_frequency_frll: 0,
            can_prp_factor_frml: 0.0,
            can_prp_factor_frll: 0.0,
            can_desired_sweep_bw_frml: 0,
            can_desired_sweep_bw_frll: 0,
        }
    }
}

impl ros2_client::Message for MrrControlMsgFR {}
