use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MeshTriangle {
    pub vertex_indices: [u32; 3],
}

impl Default for MeshTriangle {
    fn default() -> Self {
        MeshTriangle {
            vertex_indices: [0; 3],
        }
    }
}

impl ros2_client::Message for MeshTriangle {}
