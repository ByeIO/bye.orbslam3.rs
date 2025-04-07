use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReassignReportIdCmd {
    pub header: crate::std_msgs::msg::Header,
    pub confirm: bool,
    pub user_report_id: u32,
    pub use_user_report_id: bool,
}

impl Default for ReassignReportIdCmd {
    fn default() -> Self {
        ReassignReportIdCmd {
            header: crate::std_msgs::msg::Header::default(),
            confirm: false,
            user_report_id: 0,
            use_user_report_id: false,
        }
    }
}

impl ros2_client::Message for ReassignReportIdCmd {}
