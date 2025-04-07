use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HazardDetection {
    pub header: crate::std_msgs::msg::Header,
    #[serde(rename = "type")]    pub type_: u8,
}

impl HazardDetection {
    pub const BACKUP_LIMIT: u8 = 0;
    pub const BUMP: u8 = 1;
    pub const CLIFF: u8 = 2;
    pub const STALL: u8 = 3;
    pub const WHEEL_DROP: u8 = 4;
    pub const OBJECT_PROXIMITY: u8 = 5;
}

impl Default for HazardDetection {
    fn default() -> Self {
        HazardDetection {
            header: crate::std_msgs::msg::Header::default(),
            type_: 0,
        }
    }
}

impl ros2_client::Message for HazardDetection {}
