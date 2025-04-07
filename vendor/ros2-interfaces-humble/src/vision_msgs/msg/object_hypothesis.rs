use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ObjectHypothesis {
    pub class_id: ::std::string::String,
    pub score: f64,
}

impl Default for ObjectHypothesis {
    fn default() -> Self {
        ObjectHypothesis {
            class_id: ::std::string::String::new(),
            score: 0.0,
        }
    }
}

impl ros2_client::Message for ObjectHypothesis {}
