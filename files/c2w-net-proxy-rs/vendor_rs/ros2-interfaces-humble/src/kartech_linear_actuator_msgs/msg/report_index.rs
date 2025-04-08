use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReportIndex {
    pub report_index: u8,
}

impl ReportIndex {
    pub const POSITION_REPORT_INDEX: u8 = 128;
    pub const MOTOR_CURRENT_REPORT_INDEX: u8 = 129;
    pub const ENHANCED_POSITION_REPORT_INDEX: u8 = 152;
    pub const UNIQUE_DEVICE_ID_REPORTS_INDEX: u8 = 167;
    pub const SOFTWARE_REVISION_REPORT_INDEX: u8 = 229;
    pub const ZEROING_MESSAGE_REPORT_INDEX: u8 = 238;
}

impl Default for ReportIndex {
    fn default() -> Self {
        ReportIndex {
            report_index: 0,
        }
    }
}

impl ros2_client::Message for ReportIndex {}
