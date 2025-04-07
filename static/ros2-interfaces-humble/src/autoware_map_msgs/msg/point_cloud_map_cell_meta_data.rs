use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PointCloudMapCellMetaData {
    pub min_x: f32,
    pub min_y: f32,
    pub max_x: f32,
    pub max_y: f32,
}

impl Default for PointCloudMapCellMetaData {
    fn default() -> Self {
        PointCloudMapCellMetaData {
            min_x: 0.0,
            min_y: 0.0,
            max_x: 0.0,
            max_y: 0.0,
        }
    }
}

impl ros2_client::Message for PointCloudMapCellMetaData {}
