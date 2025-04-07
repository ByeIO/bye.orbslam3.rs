use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChargingState {
    pub header: crate::std_msgs::msg::Header,
    pub state: u8,
}

impl ChargingState {
    pub const CHARGE_NONE: u8 = 0;
    pub const CHARGE_RECONDITION: u8 = 1;
    pub const CHARGE_FULL: u8 = 2;
    pub const CHARGE_TRICKLE: u8 = 3;
    pub const CHARGE_WAITING: u8 = 4;
    pub const CHARGE_FAULT: u8 = 5;
}

impl Default for ChargingState {
    fn default() -> Self {
        ChargingState {
            header: crate::std_msgs::msg::Header::default(),
            state: 0,
        }
    }
}

impl ros2_client::Message for ChargingState {}
