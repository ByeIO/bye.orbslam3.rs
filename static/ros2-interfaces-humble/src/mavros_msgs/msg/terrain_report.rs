use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TerrainReport {
    pub header: crate::std_msgs::msg::Header,
    pub latitude: f64,
    pub longitude: f64,
    pub spacing: u16,
    pub terrain_height: f32,
    pub current_height: f32,
    pub pending: u16,
    pub loaded: u16,
}

impl Default for TerrainReport {
    fn default() -> Self {
        TerrainReport {
            header: crate::std_msgs::msg::Header::default(),
            latitude: 0.0,
            longitude: 0.0,
            spacing: 0,
            terrain_height: 0.0,
            current_height: 0.0,
            pending: 0,
            loaded: 0,
        }
    }
}

impl ros2_client::Message for TerrainReport {}
