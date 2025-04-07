use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OccupancyGrid {
    pub header: crate::std_msgs::msg::Header,
    pub info: crate::nav_msgs::msg::MapMetaData,
    pub data: Vec<i8>,
}

impl Default for OccupancyGrid {
    fn default() -> Self {
        OccupancyGrid {
            header: crate::std_msgs::msg::Header::default(),
            info: crate::nav_msgs::msg::MapMetaData::default(),
            data: Vec::new(),
        }
    }
}

impl ros2_client::Message for OccupancyGrid {}
