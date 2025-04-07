use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ObjectData2270 {
    pub header: crate::std_msgs::msg::Header,
    pub ibeo_header: crate::ibeo_msgs::msg::IbeoDataHeader,
    pub start_scan_timestamp: crate::builtin_interfaces::msg::Time,
    pub object_list_number: u16,
    pub number_of_objects: u16,
    pub object_list: Vec<crate::ibeo_msgs::msg::Object2270>,
}

impl Default for ObjectData2270 {
    fn default() -> Self {
        ObjectData2270 {
            header: crate::std_msgs::msg::Header::default(),
            ibeo_header: crate::ibeo_msgs::msg::IbeoDataHeader::default(),
            start_scan_timestamp: crate::builtin_interfaces::msg::Time::default(),
            object_list_number: 0,
            number_of_objects: 0,
            object_list: Vec::new(),
        }
    }
}

impl ros2_client::Message for ObjectData2270 {}
