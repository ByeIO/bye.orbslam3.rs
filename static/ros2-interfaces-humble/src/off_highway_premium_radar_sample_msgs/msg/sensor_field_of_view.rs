use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SensorFieldOfView {
    pub fov: [f32; 25],
    pub azimuth: [f32; 25],
    pub elevation_range_scaling: [f32; 11],
    pub elevation: [f32; 11],
}

impl Default for SensorFieldOfView {
    fn default() -> Self {
        SensorFieldOfView {
            fov: [0.0; 25],
            azimuth: [0.0; 25],
            elevation_range_scaling: [0.0; 11],
            elevation: [0.0; 11],
        }
    }
}

impl ros2_client::Message for SensorFieldOfView {}
