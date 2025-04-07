use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Sonar {
    pub left: f32,
    pub right: f32,
}

impl Default for Sonar {
    fn default() -> Self {
        Sonar {
            left: 0.0,
            right: 0.0,
        }
    }
}

impl ros2_client::Message for Sonar {}
