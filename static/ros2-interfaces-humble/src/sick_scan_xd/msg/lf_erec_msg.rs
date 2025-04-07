use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LFErecMsg {
    pub header: crate::std_msgs::msg::Header,
    pub fields_number: u16,
    pub fields: Vec<crate::sick_scan_xd::msg::LFErecFieldMsg>,
}

impl Default for LFErecMsg {
    fn default() -> Self {
        LFErecMsg {
            header: crate::std_msgs::msg::Header::default(),
            fields_number: 0,
            fields: Vec::new(),
        }
    }
}

impl ros2_client::Message for LFErecMsg {}
