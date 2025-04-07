use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VectorInfoCart {
    pub nr_sv: u8,
    pub error: u8,
    pub mode: u8,
    pub misc: u8,
    pub delta_x: f64,
    pub delta_y: f64,
    pub delta_z: f64,
    pub delta_vx: f32,
    pub delta_vy: f32,
    pub delta_vz: f32,
    pub azimuth: u16,
    pub elevation: i16,
    pub reference_id: u16,
    pub corr_age: u16,
    pub signal_info: u32,
}

impl Default for VectorInfoCart {
    fn default() -> Self {
        VectorInfoCart {
            nr_sv: 0,
            error: 0,
            mode: 0,
            misc: 0,
            delta_x: 0.0,
            delta_y: 0.0,
            delta_z: 0.0,
            delta_vx: 0.0,
            delta_vy: 0.0,
            delta_vz: 0.0,
            azimuth: 0,
            elevation: 0,
            reference_id: 0,
            corr_age: 0,
            signal_info: 0,
        }
    }
}

impl ros2_client::Message for VectorInfoCart {}
