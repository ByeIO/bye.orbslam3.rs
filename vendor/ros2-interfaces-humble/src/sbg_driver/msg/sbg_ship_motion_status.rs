use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SbgShipMotionStatus {
    pub heave_valid: bool,
    pub heave_vel_aided: bool,
    pub surge_sway_included: bool,
    pub period_available: bool,
    pub period_valid: bool,
    pub swell_mode: bool,
}

impl Default for SbgShipMotionStatus {
    fn default() -> Self {
        SbgShipMotionStatus {
            heave_valid: false,
            heave_vel_aided: false,
            surge_sway_included: false,
            period_available: false,
            period_valid: false,
            swell_mode: false,
        }
    }
}

impl ros2_client::Message for SbgShipMotionStatus {}
