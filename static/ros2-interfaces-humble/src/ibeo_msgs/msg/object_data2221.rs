use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ObjectData2221 {
    pub header: crate::std_msgs::msg::Header,
    pub ibeo_header: crate::ibeo_msgs::msg::IbeoDataHeader,
    pub scan_start_timestamp: crate::builtin_interfaces::msg::Time,
    pub number_of_objects: u16,
    pub object_list: Vec<crate::ibeo_msgs::msg::Object2221>,
}

impl Default for ObjectData2221 {
    fn default() -> Self {
        ObjectData2221 {
            header: crate::std_msgs::msg::Header::default(),
            ibeo_header: crate::ibeo_msgs::msg::IbeoDataHeader::default(),
            scan_start_timestamp: crate::builtin_interfaces::msg::Time::default(),
            number_of_objects: 0,
            object_list: Vec::new(),
        }
    }
}

impl ros2_client::Message for ObjectData2221 {}
