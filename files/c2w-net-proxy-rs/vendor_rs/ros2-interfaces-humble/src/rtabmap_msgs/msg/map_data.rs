use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MapData {
    pub header: crate::std_msgs::msg::Header,
    pub graph: crate::rtabmap_msgs::msg::MapGraph,
    pub nodes: Vec<crate::rtabmap_msgs::msg::Node>,
}

impl Default for MapData {
    fn default() -> Self {
        MapData {
            header: crate::std_msgs::msg::Header::default(),
            graph: crate::rtabmap_msgs::msg::MapGraph::default(),
            nodes: Vec::new(),
        }
    }
}

impl ros2_client::Message for MapData {}
