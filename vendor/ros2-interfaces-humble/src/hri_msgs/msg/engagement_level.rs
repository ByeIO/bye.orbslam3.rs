use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EngagementLevel {
    pub header: crate::std_msgs::msg::Header,
    pub level: u8,
}

impl EngagementLevel {
    pub const UNKNOWN: u8 = 0;
    pub const DISENGAGED: u8 = 1;
    pub const ENGAGING: u8 = 2;
    pub const ENGAGED: u8 = 3;
    pub const DISENGAGING: u8 = 4;
}

impl Default for EngagementLevel {
    fn default() -> Self {
        EngagementLevel {
            header: crate::std_msgs::msg::Header::default(),
            level: 0,
        }
    }
}

impl ros2_client::Message for EngagementLevel {}
