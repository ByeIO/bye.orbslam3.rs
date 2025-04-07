use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UBXNavEOE {
    pub header: crate::std_msgs::msg::Header,
    pub itow: u32,
}

impl Default for UBXNavEOE {
    fn default() -> Self {
        UBXNavEOE {
            header: crate::std_msgs::msg::Header::default(),
            itow: 0,
        }
    }
}

impl ros2_client::Message for UBXNavEOE {}
