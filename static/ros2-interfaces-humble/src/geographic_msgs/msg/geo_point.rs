use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeoPoint {
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f64,
}

impl Default for GeoPoint {
    fn default() -> Self {
        GeoPoint {
            latitude: 0.0,
            longitude: 0.0,
            altitude: 0.0,
        }
    }
}

impl ros2_client::Message for GeoPoint {}
