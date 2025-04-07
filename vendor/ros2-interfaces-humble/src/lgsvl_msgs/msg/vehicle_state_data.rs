use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VehicleStateData {
    pub header: crate::std_msgs::msg::Header,
    pub blinker_state: u8,
    pub headlight_state: u8,
    pub wiper_state: u8,
    pub current_gear: u8,
    pub vehicle_mode: u8,
    pub hand_brake_active: bool,
    pub horn_active: bool,
    pub autonomous_mode_active: bool,
}

impl VehicleStateData {
    pub const BLINKERS_OFF: u8 = 0;
    pub const BLINKERS_LEFT: u8 = 1;
    pub const BLINKERS_RIGHT: u8 = 2;
    pub const BLINKERS_HAZARD: u8 = 3;
    pub const HEADLIGHTS_OFF: u8 = 0;
    pub const HEADLIGHTS_LOW: u8 = 1;
    pub const HEADLIGHTS_HIGH: u8 = 2;
    pub const WIPERS_OFF: u8 = 0;
    pub const WIPERS_LOW: u8 = 1;
    pub const WIPERS_MED: u8 = 2;
    pub const WIPERS_HIGH: u8 = 3;
    pub const GEAR_NEUTRAL: u8 = 0;
    pub const GEAR_DRIVE: u8 = 1;
    pub const GEAR_REVERSE: u8 = 2;
    pub const GEAR_PARKING: u8 = 3;
    pub const GEAR_LOW: u8 = 4;
    pub const VEHICLE_MODE_COMPLETE_MANUAL: u8 = 0;
    pub const VEHICLE_MODE_COMPLETE_AUTO_DRIVE: u8 = 1;
    pub const VEHICLE_MODE_AUTO_STEER_ONLY: u8 = 2;
    pub const VEHICLE_MODE_AUTO_SPEED_ONLY: u8 = 3;
    pub const VEHICLE_MODE_EMERGENCY_MODE: u8 = 4;
}

impl Default for VehicleStateData {
    fn default() -> Self {
        VehicleStateData {
            header: crate::std_msgs::msg::Header::default(),
            blinker_state: 0,
            headlight_state: 0,
            wiper_state: 0,
            current_gear: 0,
            vehicle_mode: 0,
            hand_brake_active: false,
            horn_active: false,
            autonomous_mode_active: false,
        }
    }
}

impl ros2_client::Message for VehicleStateData {}
