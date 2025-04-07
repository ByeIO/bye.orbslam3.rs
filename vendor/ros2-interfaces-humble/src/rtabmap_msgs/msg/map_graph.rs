use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MapGraph {
    pub header: crate::std_msgs::msg::Header,
    pub map_to_odom: crate::geometry_msgs::msg::Transform,
    pub poses_id: Vec<i32>,
    pub poses: Vec<crate::geometry_msgs::msg::Pose>,
    pub links: Vec<crate::rtabmap_msgs::msg::Link>,
}

impl Default for MapGraph {
    fn default() -> Self {
        MapGraph {
            header: crate::std_msgs::msg::Header::default(),
            map_to_odom: crate::geometry_msgs::msg::Transform::default(),
            poses_id: Vec::new(),
            poses: Vec::new(),
            links: Vec::new(),
        }
    }
}

impl ros2_client::Message for MapGraph {}
