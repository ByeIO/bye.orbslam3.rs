use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WheelSlip {
    pub name: Vec<::std::string::String>,
    pub lateral_slip: Vec<f64>,
    pub longitudinal_slip: Vec<f64>,
}

impl Default for WheelSlip {
    fn default() -> Self {
        WheelSlip {
            name: Vec::new(),
            lateral_slip: Vec::new(),
            longitudinal_slip: Vec::new(),
        }
    }
}

impl ros2_client::Message for WheelSlip {}
