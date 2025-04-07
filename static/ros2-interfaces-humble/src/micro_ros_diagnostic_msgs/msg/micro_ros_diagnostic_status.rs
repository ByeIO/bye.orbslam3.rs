use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MicroROSDiagnosticStatus {
    pub level: u8,
    pub updater_id: u16,
    pub hardware_id: u16,
    pub number_of_values: u8,
    pub values: Vec<crate::micro_ros_diagnostic_msgs::msg::MicroROSDiagnosticKeyValue>,
}

impl MicroROSDiagnosticStatus {
    pub const OK: u8 = 0;
    pub const WARN: u8 = 1;
    pub const ERROR: u8 = 2;
    pub const STALE: u8 = 3;
}

impl Default for MicroROSDiagnosticStatus {
    fn default() -> Self {
        MicroROSDiagnosticStatus {
            level: 0,
            updater_id: 0,
            hardware_id: 0,
            number_of_values: 0,
            values: Vec::new(),
        }
    }
}

impl ros2_client::Message for MicroROSDiagnosticStatus {}
