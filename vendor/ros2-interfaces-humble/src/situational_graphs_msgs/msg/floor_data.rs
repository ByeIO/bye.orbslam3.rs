use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FloorData {
    pub header: crate::std_msgs::msg::Header,
    pub id: i32,
    pub floor_center: crate::geometry_msgs::msg::Pose,
    pub keyframe_ids: Vec<i32>,
    pub state: u8,
}

impl FloorData {
    pub const ON_FLOOR: u8 = 0;
    pub const ON_STAIRS: u8 = 1;
}

impl Default for FloorData {
    fn default() -> Self {
        FloorData {
            header: crate::std_msgs::msg::Header::default(),
            id: 0,
            floor_center: crate::geometry_msgs::msg::Pose::default(),
            keyframe_ids: Vec::new(),
            state: 0,
        }
    }
}

impl ros2_client::Message for FloorData {}
