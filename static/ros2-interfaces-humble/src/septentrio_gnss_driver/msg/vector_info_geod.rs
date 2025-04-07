use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VectorInfoGeod {
    pub nr_sv: u8,
    pub error: u8,
    pub mode: u8,
    pub misc: u8,
    pub delta_east: f64,
    pub delta_north: f64,
    pub delta_up: f64,
    pub delta_ve: f32,
    pub delta_vn: f32,
    pub delta_vu: f32,
    pub azimuth: u16,
    pub elevation: i16,
    pub reference_id: u16,
    pub corr_age: u16,
    pub signal_info: u32,
}

impl Default for VectorInfoGeod {
    fn default() -> Self {
        VectorInfoGeod {
            nr_sv: 0,
            error: 0,
            mode: 0,
            misc: 0,
            delta_east: 0.0,
            delta_north: 0.0,
            delta_up: 0.0,
            delta_ve: 0.0,
            delta_vn: 0.0,
            delta_vu: 0.0,
            azimuth: 0,
            elevation: 0,
            reference_id: 0,
            corr_age: 0,
            signal_info: 0,
        }
    }
}

impl ros2_client::Message for VectorInfoGeod {}
