use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReceiverTime {
    pub header: crate::std_msgs::msg::Header,
    pub block_header: crate::septentrio_gnss_driver::msg::BlockHeader,
    pub utc_year: i8,
    pub utc_month: i8,
    pub utc_day: i8,
    pub utc_hour: i8,
    pub utc_min: i8,
    pub utc_second: i8,
    pub delta_ls: i8,
    pub sync_level: u8,
}

impl Default for ReceiverTime {
    fn default() -> Self {
        ReceiverTime {
            header: crate::std_msgs::msg::Header::default(),
            block_header: crate::septentrio_gnss_driver::msg::BlockHeader::default(),
            utc_year: 0,
            utc_month: 0,
            utc_day: 0,
            utc_hour: 0,
            utc_min: 0,
            utc_second: 0,
            delta_ls: 0,
            sync_level: 0,
        }
    }
}

impl ros2_client::Message for ReceiverTime {}
