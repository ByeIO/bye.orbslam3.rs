use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Vertices {
    pub vertices: Vec<u32>,
}

impl Default for Vertices {
    fn default() -> Self {
        Vertices {
            vertices: Vec::new(),
        }
    }
}

impl ros2_client::Message for Vertices {}
