use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GearReject {
    pub value: u8,
}

impl GearReject {
    pub const NONE: u8 = 0;
    pub const FAULT: u8 = 1;
    pub const UNSUPPORTED: u8 = 2;
    pub const SHIFT_IN_PROGRESS: u8 = 3;
    pub const OVERRIDE: u8 = 4;
    pub const BRAKE_HOLD: u8 = 5;
    pub const VEHICLE_SPEED: u8 = 6;
    pub const VEHICLE: u8 = 7;
}

impl Default for GearReject {
    fn default() -> Self {
        GearReject {
            value: 0,
        }
    }
}

impl ros2_client::Message for GearReject {}
