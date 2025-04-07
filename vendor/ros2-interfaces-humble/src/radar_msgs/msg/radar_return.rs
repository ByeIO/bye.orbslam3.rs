use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RadarReturn {
    pub range: f32,
    pub azimuth: f32,
    pub elevation: f32,
    pub doppler_velocity: f32,
    pub amplitude: f32,
}

impl Default for RadarReturn {
    fn default() -> Self {
        RadarReturn {
            range: 0.0,
            azimuth: 0.0,
            elevation: 0.0,
            doppler_velocity: 0.0,
            amplitude: 0.0,
        }
    }
}

impl ros2_client::Message for RadarReturn {}
