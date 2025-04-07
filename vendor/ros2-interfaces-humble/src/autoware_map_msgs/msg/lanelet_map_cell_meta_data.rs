use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LaneletMapCellMetaData {
    pub cell_id: ::std::string::String,
    pub min_x: f64,
    pub max_x: f64,
    pub min_y: f64,
    pub max_y: f64,
}

impl Default for LaneletMapCellMetaData {
    fn default() -> Self {
        LaneletMapCellMetaData {
            cell_id: ::std::string::String::new(),
            min_x: 0.0,
            max_x: 0.0,
            min_y: 0.0,
            max_y: 0.0,
        }
    }
}

impl ros2_client::Message for LaneletMapCellMetaData {}
