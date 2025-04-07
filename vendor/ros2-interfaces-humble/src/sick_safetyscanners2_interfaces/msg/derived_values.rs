use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DerivedValues {
    pub multiplication_factor: u16,
    pub number_of_beams: u16,
    pub scan_time: u16,
    pub start_angle: f32,
    pub angular_beam_resolution: f32,
    pub interbeam_period: u32,
}

impl Default for DerivedValues {
    fn default() -> Self {
        DerivedValues {
            multiplication_factor: 0,
            number_of_beams: 0,
            scan_time: 0,
            start_angle: 0.0,
            angular_beam_resolution: 0.0,
            interbeam_period: 0,
        }
    }
}

impl ros2_client::Message for DerivedValues {}
