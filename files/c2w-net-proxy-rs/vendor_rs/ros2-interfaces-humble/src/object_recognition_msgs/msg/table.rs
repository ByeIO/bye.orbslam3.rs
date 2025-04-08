use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Table {
    pub header: crate::std_msgs::msg::Header,
    pub pose: crate::geometry_msgs::msg::Pose,
    pub convex_hull: Vec<crate::geometry_msgs::msg::Point>,
}

impl Default for Table {
    fn default() -> Self {
        Table {
            header: crate::std_msgs::msg::Header::default(),
            pose: crate::geometry_msgs::msg::Pose::default(),
            convex_hull: Vec::new(),
        }
    }
}

impl ros2_client::Message for Table {}
