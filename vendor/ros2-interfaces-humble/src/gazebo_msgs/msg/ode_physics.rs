use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ODEPhysics {
    pub auto_disable_bodies: bool,
    pub sor_pgs_precon_iters: u32,
    pub sor_pgs_iters: u32,
    pub sor_pgs_w: f64,
    pub sor_pgs_rms_error_tol: f64,
    pub contact_surface_layer: f64,
    pub contact_max_correcting_vel: f64,
    pub cfm: f64,
    pub erp: f64,
    pub max_contacts: u32,
}

impl Default for ODEPhysics {
    fn default() -> Self {
        ODEPhysics {
            auto_disable_bodies: false,
            sor_pgs_precon_iters: 0,
            sor_pgs_iters: 0,
            sor_pgs_w: 0.0,
            sor_pgs_rms_error_tol: 0.0,
            contact_surface_layer: 0.0,
            contact_max_correcting_vel: 0.0,
            cfm: 0.0,
            erp: 0.0,
            max_contacts: 0,
        }
    }
}

impl ros2_client::Message for ODEPhysics {}
