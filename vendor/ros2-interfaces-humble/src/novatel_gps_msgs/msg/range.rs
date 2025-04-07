use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Range {
    pub header: crate::std_msgs::msg::Header,
    pub novatel_msg_header: crate::novatel_gps_msgs::msg::NovatelMessageHeader,
    pub numb_of_observ: i32,
    pub info: Vec<crate::novatel_gps_msgs::msg::RangeInformation>,
}

impl Default for Range {
    fn default() -> Self {
        Range {
            header: crate::std_msgs::msg::Header::default(),
            novatel_msg_header: crate::novatel_gps_msgs::msg::NovatelMessageHeader::default(),
            numb_of_observ: 0,
            info: Vec::new(),
        }
    }
}

impl ros2_client::Message for Range {}
