use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Edges {
    pub node_ids: Vec<u32>,
    pub weights: Vec<f64>,
}

impl Default for Edges {
    fn default() -> Self {
        Edges {
            node_ids: Vec::new(),
            weights: Vec::new(),
        }
    }
}

impl ros2_client::Message for Edges {}
