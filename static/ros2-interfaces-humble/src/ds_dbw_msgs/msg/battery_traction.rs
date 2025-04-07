use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BatteryTraction {
    pub header: crate::std_msgs::msg::Header,
    pub state_of_charge: f32,
    pub voltage: f32,
    pub current: f32,
    pub temperature: f32,
    pub status: u8,
}

impl BatteryTraction {
    pub const STATUS_UNKNOWN: u8 = 0;
    pub const STATUS_NOT_CHARGING: u8 = 1;
    pub const STATUS_CHARGING: u8 = 2;
    pub const STATUS_COMPLETE: u8 = 3;
    pub const STATUS_FAULT: u8 = 7;
}

impl Default for BatteryTraction {
    fn default() -> Self {
        BatteryTraction {
            header: crate::std_msgs::msg::Header::default(),
            state_of_charge: 0.0,
            voltage: 0.0,
            current: 0.0,
            temperature: 0.0,
            status: 0,
        }
    }
}

impl ros2_client::Message for BatteryTraction {}
