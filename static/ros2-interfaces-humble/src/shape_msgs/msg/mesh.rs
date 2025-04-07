use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Mesh {
    pub triangles: Vec<crate::shape_msgs::msg::MeshTriangle>,
    pub vertices: Vec<crate::geometry_msgs::msg::Point>,
}

impl Default for Mesh {
    fn default() -> Self {
        Mesh {
            triangles: Vec::new(),
            vertices: Vec::new(),
        }
    }
}

impl ros2_client::Message for Mesh {}
