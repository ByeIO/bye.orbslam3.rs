use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UBXSecUniqid {
    pub header: crate::std_msgs::msg::Header,
    pub version: u8,
    pub unique_id: [u8; 5],
}

impl Default for UBXSecUniqid {
    fn default() -> Self {
        UBXSecUniqid {
            header: crate::std_msgs::msg::Header::default(),
            version: 0,
            unique_id: [0; 5],
        }
    }
}

impl ros2_client::Message for UBXSecUniqid {}
