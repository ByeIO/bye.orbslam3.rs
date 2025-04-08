use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RouteSegment {
    pub id: u32,
    #[serde(rename = "type")]    pub type_: u8,
    pub orientation: u8,
    pub motion_type: u8,
    pub start: crate::geometry_msgs::msg::Pose,
    pub end: crate::geometry_msgs::msg::Pose,
    pub center: crate::geometry_msgs::msg::Pose,
    pub level: i8,
}

impl Default for RouteSegment {
    fn default() -> Self {
        RouteSegment {
            id: 0,
            type_: 0,
            orientation: 0,
            motion_type: 0,
            start: crate::geometry_msgs::msg::Pose::default(),
            end: crate::geometry_msgs::msg::Pose::default(),
            center: crate::geometry_msgs::msg::Pose::default(),
            level: 0,
        }
    }
}

impl ros2_client::Message for RouteSegment {}
