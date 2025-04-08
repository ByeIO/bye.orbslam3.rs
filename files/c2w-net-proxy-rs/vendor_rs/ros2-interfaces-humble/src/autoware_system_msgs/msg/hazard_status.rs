use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HazardStatus {
    pub level: u8,
    pub emergency: bool,
    pub emergency_holding: bool,
    pub diag_no_fault: Vec<crate::diagnostic_msgs::msg::DiagnosticStatus>,
    pub diag_safe_fault: Vec<crate::diagnostic_msgs::msg::DiagnosticStatus>,
    pub diag_latent_fault: Vec<crate::diagnostic_msgs::msg::DiagnosticStatus>,
    pub diag_single_point_fault: Vec<crate::diagnostic_msgs::msg::DiagnosticStatus>,
}

impl HazardStatus {
    pub const NO_FAULT: u8 = 0;
    pub const SAFE_FAULT: u8 = 1;
    pub const LATENT_FAULT: u8 = 2;
    pub const SINGLE_POINT_FAULT: u8 = 3;
}

impl Default for HazardStatus {
    fn default() -> Self {
        HazardStatus {
            level: 0,
            emergency: false,
            emergency_holding: false,
            diag_no_fault: Vec::new(),
            diag_safe_fault: Vec::new(),
            diag_latent_fault: Vec::new(),
            diag_single_point_fault: Vec::new(),
        }
    }
}

impl ros2_client::Message for HazardStatus {}
