use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ODEJointProperties {
    pub damping: Vec<f64>,
    pub hi_stop: Vec<f64>,
    pub lo_stop: Vec<f64>,
    pub erp: Vec<f64>,
    pub cfm: Vec<f64>,
    pub stop_erp: Vec<f64>,
    pub stop_cfm: Vec<f64>,
    pub fudge_factor: Vec<f64>,
    pub fmax: Vec<f64>,
    pub vel: Vec<f64>,
}

impl Default for ODEJointProperties {
    fn default() -> Self {
        ODEJointProperties {
            damping: Vec::new(),
            hi_stop: Vec::new(),
            lo_stop: Vec::new(),
            erp: Vec::new(),
            cfm: Vec::new(),
            stop_erp: Vec::new(),
            stop_cfm: Vec::new(),
            fudge_factor: Vec::new(),
            fmax: Vec::new(),
            vel: Vec::new(),
        }
    }
}

impl ros2_client::Message for ODEJointProperties {}
