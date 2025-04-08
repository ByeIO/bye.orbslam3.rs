use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LineSegment {
    pub id: u32,
    pub p0: crate::geometry_msgs::msg::Point,
    pub p1: crate::geometry_msgs::msg::Point,
}

impl Default for LineSegment {
    fn default() -> Self {
        LineSegment {
            id: 0,
            p0: crate::geometry_msgs::msg::Point::default(),
            p1: crate::geometry_msgs::msg::Point::default(),
        }
    }
}

impl ros2_client::Message for LineSegment {}
