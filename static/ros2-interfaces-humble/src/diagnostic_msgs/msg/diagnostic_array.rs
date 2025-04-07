use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DiagnosticArray {
    pub header: crate::std_msgs::msg::Header,
    pub status: Vec<crate::diagnostic_msgs::msg::DiagnosticStatus>,
}

impl Default for DiagnosticArray {
    fn default() -> Self {
        DiagnosticArray {
            header: crate::std_msgs::msg::Header::default(),
            status: Vec::new(),
        }
    }
}

impl ros2_client::Message for DiagnosticArray {}
