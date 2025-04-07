use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActiveMonitoringCase {
    pub monitoring_case_1: u8,
    pub monitoring_case_2: u8,
    pub monitoring_case_3: u8,
    pub monitoring_case_4: u8,
}

impl Default for ActiveMonitoringCase {
    fn default() -> Self {
        ActiveMonitoringCase {
            monitoring_case_1: 0,
            monitoring_case_2: 0,
            monitoring_case_3: 0,
            monitoring_case_4: 0,
        }
    }
}

impl ros2_client::Message for ActiveMonitoringCase {}
