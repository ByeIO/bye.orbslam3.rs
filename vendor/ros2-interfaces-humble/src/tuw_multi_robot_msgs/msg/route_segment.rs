use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RouteSegment {
    pub segment_id: i32,
    pub preconditions: Vec<crate::tuw_multi_robot_msgs::msg::RoutePrecondition>,
    pub start: crate::geometry_msgs::msg::Pose,
    pub end: crate::geometry_msgs::msg::Pose,
    pub width: f32,
}

impl Default for RouteSegment {
    fn default() -> Self {
        RouteSegment {
            segment_id: 0,
            preconditions: Vec::new(),
            start: crate::geometry_msgs::msg::Pose::default(),
            end: crate::geometry_msgs::msg::Pose::default(),
            width: 0.0,
        }
    }
}

impl ros2_client::Message for RouteSegment {}
