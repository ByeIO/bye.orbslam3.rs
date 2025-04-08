use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LaneletRoute {
    pub header: crate::std_msgs::msg::Header,
    pub start_pose: crate::geometry_msgs::msg::Pose,
    pub goal_pose: crate::geometry_msgs::msg::Pose,
    pub segments: Vec<crate::autoware_planning_msgs::msg::LaneletSegment>,
    pub uuid: crate::unique_identifier_msgs::msg::UUID,
    pub allow_modification: bool,
}

impl Default for LaneletRoute {
    fn default() -> Self {
        LaneletRoute {
            header: crate::std_msgs::msg::Header::default(),
            start_pose: crate::geometry_msgs::msg::Pose::default(),
            goal_pose: crate::geometry_msgs::msg::Pose::default(),
            segments: Vec::new(),
            uuid: crate::unique_identifier_msgs::msg::UUID::default(),
            allow_modification: false,
        }
    }
}

impl ros2_client::Message for LaneletRoute {}
