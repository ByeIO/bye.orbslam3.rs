use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MonitoringCase {
    pub monitoring_case_number: i32,
    pub fields: Vec<i32>,
    pub fields_valid: Vec<bool>,
}

impl Default for MonitoringCase {
    fn default() -> Self {
        MonitoringCase {
            monitoring_case_number: 0,
            fields: Vec::new(),
            fields_valid: Vec::new(),
        }
    }
}

impl ros2_client::Message for MonitoringCase {}
