use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ObjectData2280 {
    pub header: crate::std_msgs::msg::Header,
    pub ibeo_header: crate::ibeo_msgs::msg::IbeoDataHeader,
    pub mid_scan_timestamp: crate::builtin_interfaces::msg::Time,
    pub number_of_objects: u16,
    pub objects: Vec<crate::ibeo_msgs::msg::Object2280>,
}

impl Default for ObjectData2280 {
    fn default() -> Self {
        ObjectData2280 {
            header: crate::std_msgs::msg::Header::default(),
            ibeo_header: crate::ibeo_msgs::msg::IbeoDataHeader::default(),
            mid_scan_timestamp: crate::builtin_interfaces::msg::Time::default(),
            number_of_objects: 0,
            objects: Vec::new(),
        }
    }
}

impl ros2_client::Message for ObjectData2280 {}
