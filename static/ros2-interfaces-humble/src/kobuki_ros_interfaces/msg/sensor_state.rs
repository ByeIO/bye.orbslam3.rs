use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SensorState {
    pub header: crate::std_msgs::msg::Header,
    pub time_stamp: u16,
    pub bumper: u8,
    pub wheel_drop: u8,
    pub cliff: u8,
    pub left_encoder: u16,
    pub right_encoder: u16,
    pub left_pwm: i8,
    pub right_pwm: i8,
    pub buttons: u8,
    pub charger: u8,
    pub battery: u8,
    pub bottom: Vec<u16>,
    pub current: Vec<u8>,
    pub over_current: u8,
    pub digital_input: u16,
    pub analog_input: Vec<u16>,
}

impl SensorState {
    pub const BUMPER_RIGHT: u8 = 1;
    pub const BUMPER_CENTRE: u8 = 2;
    pub const BUMPER_LEFT: u8 = 4;
    pub const WHEEL_DROP_RIGHT: u8 = 1;
    pub const WHEEL_DROP_LEFT: u8 = 2;
    pub const CLIFF_RIGHT: u8 = 1;
    pub const CLIFF_CENTRE: u8 = 2;
    pub const CLIFF_LEFT: u8 = 4;
    pub const BUTTON0: u8 = 1;
    pub const BUTTON1: u8 = 2;
    pub const BUTTON2: u8 = 4;
    pub const DISCHARGING: u8 = 0;
    pub const DOCKING_CHARGED: u8 = 2;
    pub const DOCKING_CHARGING: u8 = 6;
    pub const ADAPTER_CHARGED: u8 = 18;
    pub const ADAPTER_CHARGING: u8 = 22;
    pub const OVER_CURRENT_LEFT_WHEEL: u8 = 1;
    pub const OVER_CURRENT_RIGHT_WHEEL: u8 = 2;
    pub const OVER_CURRENT_BOTH_WHEELS: u8 = 3;
    pub const DIGITAL_INPUT0: u8 = 1;
    pub const DIGITAL_INPUT1: u8 = 2;
    pub const DIGITAL_INPUT2: u8 = 4;
    pub const DIGITAL_INPUT3: u8 = 8;
    pub const DB25_TEST_BOARD_CONNECTED: u8 = 64;
}

impl Default for SensorState {
    fn default() -> Self {
        SensorState {
            header: crate::std_msgs::msg::Header::default(),
            time_stamp: 0,
            bumper: 0,
            wheel_drop: 0,
            cliff: 0,
            left_encoder: 0,
            right_encoder: 0,
            left_pwm: 0,
            right_pwm: 0,
            buttons: 0,
            charger: 0,
            battery: 0,
            bottom: Vec::new(),
            current: Vec::new(),
            over_current: 0,
            digital_input: 0,
            analog_input: Vec::new(),
        }
    }
}

impl ros2_client::Message for SensorState {}
