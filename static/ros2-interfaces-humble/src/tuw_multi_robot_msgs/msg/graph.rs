use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Graph {
    pub header: crate::std_msgs::msg::Header,
    pub origin: crate::geometry_msgs::msg::Pose,
    pub vertices: Vec<crate::tuw_multi_robot_msgs::msg::Vertex>,
}

impl Default for Graph {
    fn default() -> Self {
        Graph {
            header: crate::std_msgs::msg::Header::default(),
            origin: crate::geometry_msgs::msg::Pose::default(),
            vertices: Vec::new(),
        }
    }
}

impl ros2_client::Message for Graph {}
