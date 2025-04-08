use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LaserEcho {
    pub echoes: Vec<f32>,
}

impl Default for LaserEcho {
    fn default() -> Self {
        LaserEcho {
            echoes: Vec::new(),
        }
    }
}

impl ros2_client::Message for LaserEcho {}
