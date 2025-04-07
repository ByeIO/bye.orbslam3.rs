use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DashboardState {
    pub diagnostics_toplevel_state: crate::diagnostic_msgs::msg::DiagnosticStatus,
    pub power_state: crate::cob_msgs::msg::PowerState,
    pub emergency_stop_state: crate::cob_msgs::msg::EmergencyStopState,
}

impl Default for DashboardState {
    fn default() -> Self {
        DashboardState {
            diagnostics_toplevel_state: crate::diagnostic_msgs::msg::DiagnosticStatus::default(),
            power_state: crate::cob_msgs::msg::PowerState::default(),
            emergency_stop_state: crate::cob_msgs::msg::EmergencyStopState::default(),
        }
    }
}

impl ros2_client::Message for DashboardState {}
