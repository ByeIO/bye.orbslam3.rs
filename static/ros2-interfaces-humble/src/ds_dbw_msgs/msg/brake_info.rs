use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BrakeInfo {
    pub header: crate::std_msgs::msg::Header,
    pub brake_torque_pedal: f32,
    pub brake_torque_request: f32,
    pub brake_torque_actual: f32,
    pub brake_pedal_qf: crate::ds_dbw_msgs::msg::Quality,
    pub brake_vacuum: f32,
    pub abs_active: bool,
    pub abs_enabled: bool,
    pub esc_active: bool,
    pub esc_enabled: bool,
    pub trac_active: bool,
    pub trac_enabled: bool,
}

impl Default for BrakeInfo {
    fn default() -> Self {
        BrakeInfo {
            header: crate::std_msgs::msg::Header::default(),
            brake_torque_pedal: 0.0,
            brake_torque_request: 0.0,
            brake_torque_actual: 0.0,
            brake_pedal_qf: crate::ds_dbw_msgs::msg::Quality::default(),
            brake_vacuum: 0.0,
            abs_active: false,
            abs_enabled: false,
            esc_active: false,
            esc_enabled: false,
            trac_active: false,
            trac_enabled: false,
        }
    }
}

impl ros2_client::Message for BrakeInfo {}
