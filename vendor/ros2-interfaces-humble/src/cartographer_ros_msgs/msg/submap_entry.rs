use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubmapEntry {
    pub trajectory_id: i32,
    pub submap_index: i32,
    pub submap_version: i32,
    pub pose: crate::geometry_msgs::msg::Pose,
    pub is_frozen: bool,
}

impl Default for SubmapEntry {
    fn default() -> Self {
        SubmapEntry {
            trajectory_id: 0,
            submap_index: 0,
            submap_version: 0,
            pose: crate::geometry_msgs::msg::Pose::default(),
            is_frozen: false,
        }
    }
}

impl ros2_client::Message for SubmapEntry {}
