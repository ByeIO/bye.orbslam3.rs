use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Path {
    pub header: crate::std_msgs::msg::Header,
    pub points: Vec<crate::autoware_planning_msgs::msg::PathPoint>,
    pub left_bound: Vec<crate::geometry_msgs::msg::Point>,
    pub right_bound: Vec<crate::geometry_msgs::msg::Point>,
}

impl Default for Path {
    fn default() -> Self {
        Path {
            header: crate::std_msgs::msg::Header::default(),
            points: Vec::new(),
            left_bound: Vec::new(),
            right_bound: Vec::new(),
        }
    }
}

impl ros2_client::Message for Path {}
