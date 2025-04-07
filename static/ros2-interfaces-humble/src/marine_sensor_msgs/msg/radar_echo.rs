use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RadarEcho {
    pub echoes: Vec<f32>,
}

impl Default for RadarEcho {
    fn default() -> Self {
        RadarEcho {
            echoes: Vec::new(),
        }
    }
}

impl ros2_client::Message for RadarEcho {}
