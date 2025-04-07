use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectedMapInfo {
    pub frame_id: ::std::string::String,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub min_z: f64,
    pub max_z: f64,
}

impl Default for ProjectedMapInfo {
    fn default() -> Self {
        ProjectedMapInfo {
            frame_id: ::std::string::String::new(),
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0,
            min_z: 0.0,
            max_z: 0.0,
        }
    }
}

impl ros2_client::Message for ProjectedMapInfo {}
