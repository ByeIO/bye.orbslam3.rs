use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BasicMsg {
    pub c: f32,
}

impl Default for BasicMsg {
    fn default() -> Self {
        BasicMsg {
            c: 0.0,
        }
    }
}

impl ros2_client::Message for BasicMsg {}
