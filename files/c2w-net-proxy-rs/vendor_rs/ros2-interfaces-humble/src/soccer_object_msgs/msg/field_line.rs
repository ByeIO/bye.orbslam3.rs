use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FieldLine {
    pub header: crate::std_msgs::msg::Header,
    pub start: crate::geometry_msgs::msg::Point,
    pub end: crate::geometry_msgs::msg::Point,
}

impl Default for FieldLine {
    fn default() -> Self {
        FieldLine {
            header: crate::std_msgs::msg::Header::default(),
            start: crate::geometry_msgs::msg::Point::default(),
            end: crate::geometry_msgs::msg::Point::default(),
        }
    }
}

impl ros2_client::Message for FieldLine {}
