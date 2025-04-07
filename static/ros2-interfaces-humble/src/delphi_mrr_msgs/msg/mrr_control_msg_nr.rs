use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MrrControlMsgNR {
    pub header: crate::std_msgs::msg::Header,
    pub can_stop_frequency_nrml: u16,
    pub can_prp_factor_nrml: u16,
    pub can_desired_sweep_bw_nrml: u8,
    pub can_radiation_ctrl: bool,
    pub can_stop_frequency_nrll: u16,
    pub can_prp_factor_nrll: u16,
    pub can_desired_sweep_bw_nrll: u8,
}

impl Default for MrrControlMsgNR {
    fn default() -> Self {
        MrrControlMsgNR {
            header: crate::std_msgs::msg::Header::default(),
            can_stop_frequency_nrml: 0,
            can_prp_factor_nrml: 0,
            can_desired_sweep_bw_nrml: 0,
            can_radiation_ctrl: false,
            can_stop_frequency_nrll: 0,
            can_prp_factor_nrll: 0,
            can_desired_sweep_bw_nrll: 0,
        }
    }
}

impl ros2_client::Message for MrrControlMsgNR {}
