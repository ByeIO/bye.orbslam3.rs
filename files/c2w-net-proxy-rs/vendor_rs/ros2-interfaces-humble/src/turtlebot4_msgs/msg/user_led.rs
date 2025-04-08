use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserLed {
    pub led: u8,
    pub color: u8,
    pub blink_period: u32,
    pub duty_cycle: f64,
}

impl UserLed {
    pub const USER_LED_1: u8 = 0;
    pub const USER_LED_2: u8 = 1;
    pub const COLOR_OFF: u8 = 0;
    pub const COLOR_GREEN: u8 = 1;
    pub const COLOR_RED: u8 = 2;
    pub const COLOR_YELLOW: u8 = 3;
}

impl Default for UserLed {
    fn default() -> Self {
        UserLed {
            led: 0,
            color: 0,
            blink_period: 0,
            duty_cycle: 0.0,
        }
    }
}

impl ros2_client::Message for UserLed {}
