use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeometryGraph {
    pub header: crate::std_msgs::msg::Header,
    pub nodes: Vec<crate::geometry_msgs::msg::Point>,
    pub edges: Vec<crate::graph_msgs::msg::Edges>,
}

impl Default for GeometryGraph {
    fn default() -> Self {
        GeometryGraph {
            header: crate::std_msgs::msg::Header::default(),
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }
}

impl ros2_client::Message for GeometryGraph {}
