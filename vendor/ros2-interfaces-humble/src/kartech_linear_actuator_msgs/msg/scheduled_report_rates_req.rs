use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScheduledReportRatesReq {
    pub header: crate::std_msgs::msg::Header,
    pub confirm: bool,
    pub index_1: crate::kartech_linear_actuator_msgs::msg::ReportIndex,
    pub index_1_report_time: u16,
    pub index_2: crate::kartech_linear_actuator_msgs::msg::ReportIndex,
    pub index_2_report_time: u16,
}

impl Default for ScheduledReportRatesReq {
    fn default() -> Self {
        ScheduledReportRatesReq {
            header: crate::std_msgs::msg::Header::default(),
            confirm: false,
            index_1: crate::kartech_linear_actuator_msgs::msg::ReportIndex::default(),
            index_1_report_time: 0,
            index_2: crate::kartech_linear_actuator_msgs::msg::ReportIndex::default(),
            index_2_report_time: 0,
        }
    }
}

impl ros2_client::Message for ScheduledReportRatesReq {}
