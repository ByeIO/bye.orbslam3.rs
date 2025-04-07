use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourceData {
    pub name: ::std::string::String,
    pub position: f64,
    pub velocity: f64,
    pub effort: f64,
    pub command: f64,
}

impl Default for ResourceData {
    fn default() -> Self {
        ResourceData {
            name: ::std::string::String::new(),
            position: 0.0,
            velocity: 0.0,
            effort: 0.0,
            command: 0.0,
        }
    }
}

impl ros2_client::Message for ResourceData {}
