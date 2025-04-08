use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PingInfo {
    pub frequency: f32,
    pub sound_speed: f32,
    pub tx_beamwidths: Vec<f32>,
    pub rx_beamwidths: Vec<f32>,
}

impl Default for PingInfo {
    fn default() -> Self {
        PingInfo {
            frequency: 0.0,
            sound_speed: 0.0,
            tx_beamwidths: Vec::new(),
            rx_beamwidths: Vec::new(),
        }
    }
}

impl ros2_client::Message for PingInfo {}
