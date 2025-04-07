use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Segment {
    pub id: u64, // default: 0
    pub label: u32, // default: 0
    pub angular_distance: f64, // default: 0.0
    pub last_point_prior_segment: crate::geometry_msgs::msg::Point,
    pub first_point_next_segment: crate::geometry_msgs::msg::Point,
    pub points: Vec<crate::geometry_msgs::msg::Point>,
}

impl Segment {
    pub const BACKGROUND: u32 = 0;
    pub const PERSON: u32 = 1;
    pub const PERSON_CANE: u32 = 2;
    pub const PERSON_WHEEL_CHAIR: u32 = 3;
}

impl Default for Segment {
    fn default() -> Self {
        Segment {
            id: 0,
            label: 0,
            angular_distance: 0.0,
            last_point_prior_segment: crate::geometry_msgs::msg::Point::default(),
            first_point_next_segment: crate::geometry_msgs::msg::Point::default(),
            points: Vec::new(),
        }
    }
}

impl ros2_client::Message for Segment {}
