use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClientExpandMapVisualization {
    pub timestamp: crate::builtin_interfaces::msg::Time,
    pub visualization_id: u64,
    pub zones: Vec<crate::bosch_locator_bridge::msg::ClientExpandMapOverwriteZoneInformation>,
    pub prior_map_poses: crate::geometry_msgs::msg::PoseArray,
    pub prior_map_pose_types: Vec<i32>,
}

impl Default for ClientExpandMapVisualization {
    fn default() -> Self {
        ClientExpandMapVisualization {
            timestamp: crate::builtin_interfaces::msg::Time::default(),
            visualization_id: 0,
            zones: Vec::new(),
            prior_map_poses: crate::geometry_msgs::msg::PoseArray::default(),
            prior_map_pose_types: Vec::new(),
        }
    }
}

impl ros2_client::Message for ClientExpandMapVisualization {}
