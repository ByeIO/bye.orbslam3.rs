use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GPS {
    pub stamp: f64,
    pub longitude: f64,
    pub latitude: f64,
    pub altitude: f64,
    pub error: f64,
    pub bearing: f64,
}

impl Default for GPS {
    fn default() -> Self {
        GPS {
            stamp: 0.0,
            longitude: 0.0,
            latitude: 0.0,
            altitude: 0.0,
            error: 0.0,
            bearing: 0.0,
        }
    }
}

impl ros2_client::Message for GPS {}
