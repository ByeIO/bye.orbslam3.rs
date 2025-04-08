use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MisalignmentPacket {
    pub theta: f32,
    pub theta_variance: f32,
    pub phi: f32,
    pub phi_variance: f32,
    pub phi_eme: f32,
    pub phi_eme_variance: f32,
    pub status: u16,
    pub status_eme: u16,
    pub percent_negative_theta: f32,
    pub min_theta_sos: f32,
    pub max_theta_sos: f32,
    pub theta_sos_variance: f32,
    pub theta_sos_mean: f32,
    pub min_phi_sos: f32,
    pub max_phi_sos: f32,
    pub phi_sos_variance: f32,
    pub phi_sos_mean: f32,
    pub phi_sos_spread: f32,
    pub num_sos: u16,
    pub num_eme: u16,
}

impl Default for MisalignmentPacket {
    fn default() -> Self {
        MisalignmentPacket {
            theta: 0.0,
            theta_variance: 0.0,
            phi: 0.0,
            phi_variance: 0.0,
            phi_eme: 0.0,
            phi_eme_variance: 0.0,
            status: 0,
            status_eme: 0,
            percent_negative_theta: 0.0,
            min_theta_sos: 0.0,
            max_theta_sos: 0.0,
            theta_sos_variance: 0.0,
            theta_sos_mean: 0.0,
            min_phi_sos: 0.0,
            max_phi_sos: 0.0,
            phi_sos_variance: 0.0,
            phi_sos_mean: 0.0,
            phi_sos_spread: 0.0,
            num_sos: 0,
            num_eme: 0,
        }
    }
}

impl ros2_client::Message for MisalignmentPacket {}
