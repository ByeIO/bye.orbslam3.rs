use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ESCStatus {
    pub header: crate::std_msgs::msg::Header,
    pub esc_status: Vec<crate::mavros_msgs::msg::ESCStatusItem>,
}

impl Default for ESCStatus {
    fn default() -> Self {
        ESCStatus {
            header: crate::std_msgs::msg::Header::default(),
            esc_status: Vec::new(),
        }
    }
}

impl ros2_client::Message for ESCStatus {}
