use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReportPollReq {
    pub header: crate::std_msgs::msg::Header,
    pub confirm: bool,
    pub report_indices: Vec<crate::kartech_linear_actuator_msgs::msg::ReportIndex>,
}

impl Default for ReportPollReq {
    fn default() -> Self {
        ReportPollReq {
            header: crate::std_msgs::msg::Header::default(),
            confirm: false,
            report_indices: Vec::new(),
        }
    }
}

impl ros2_client::Message for ReportPollReq {}
