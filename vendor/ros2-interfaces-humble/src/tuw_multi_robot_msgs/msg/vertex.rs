use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Vertex {
    pub id: u32,
    pub valid: bool,
    pub path: Vec<crate::geometry_msgs::msg::Point>,
    pub weight: u32,
    pub width: f32,
    pub successors: Vec<u32>,
    pub predecessors: Vec<u32>,
}

impl Default for Vertex {
    fn default() -> Self {
        Vertex {
            id: 0,
            valid: false,
            path: Vec::new(),
            weight: 0,
            width: 0.0,
            successors: Vec::new(),
            predecessors: Vec::new(),
        }
    }
}

impl ros2_client::Message for Vertex {}
