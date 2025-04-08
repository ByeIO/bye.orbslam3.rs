use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ESCInfo {
    pub header: crate::std_msgs::msg::Header,
    pub counter: u16,
    pub count: u8,
    pub connection_type: u8,
    pub info: u8,
    pub esc_info: Vec<crate::mavros_msgs::msg::ESCInfoItem>,
}

impl Default for ESCInfo {
    fn default() -> Self {
        ESCInfo {
            header: crate::std_msgs::msg::Header::default(),
            counter: 0,
            count: 0,
            connection_type: 0,
            info: 0,
            esc_info: Vec::new(),
        }
    }
}

impl ros2_client::Message for ESCInfo {}
