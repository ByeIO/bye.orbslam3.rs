use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct State {
    pub header: crate::std_msgs::msg::Header,
    pub angular_velocity_zeroed: crate::geometry_msgs::msg::Vector3,
    pub angular_velocity_raw: crate::geometry_msgs::msg::Vector3,
    pub angular_velocity_covariance: [f64; 9],
    pub linear_acceleration_zeroed: crate::geometry_msgs::msg::Vector3,
    pub linear_acceleration_raw: crate::geometry_msgs::msg::Vector3,
    pub linear_acceleration_covariance: [f64; 9],
    pub nunchuk_acceleration_zeroed: crate::geometry_msgs::msg::Vector3,
    pub nunchuk_acceleration_raw: crate::geometry_msgs::msg::Vector3,
    pub nunchuk_joystick_zeroed: [f32; 2],
    pub nunchuk_joystick_raw: [f32; 2],
    pub buttons: [bool; 11],
    pub nunchuk_buttons: [bool; 2],
    pub leds: [bool; 4],
    pub rumble: bool,
    pub ir_tracking: Vec<crate::wiimote_msgs::msg::IrSourceInfo>,
    pub raw_battery: f32,
    pub percent_battery: f32,
    pub zeroing_time: crate::builtin_interfaces::msg::Time,
    pub errors: u64,
}

impl State {
    pub const INVALID: i8 = -1;
    pub const INVALID_FLOAT: f32 = -1.0;
    pub const MSG_BTN_1: i8 = 0;
    pub const MSG_BTN_2: i8 = 1;
    pub const MSG_BTN_A: i8 = 2;
    pub const MSG_BTN_B: i8 = 3;
    pub const MSG_BTN_PLUS: i8 = 4;
    pub const MSG_BTN_MINUS: i8 = 5;
    pub const MSG_BTN_LEFT: i8 = 6;
    pub const MSG_BTN_RIGHT: i8 = 7;
    pub const MSG_BTN_UP: i8 = 8;
    pub const MSG_BTN_DOWN: i8 = 9;
    pub const MSG_BTN_HOME: i8 = 10;
    pub const MSG_BTN_Z: i8 = 0;
    pub const MSG_BTN_C: i8 = 1;
    pub const MSG_CLASSIC_BTN_X: i8 = 0;
    pub const MSG_CLASSIC_BTN_Y: i8 = 1;
    pub const MSG_CLASSIC_BTN_A: i8 = 2;
    pub const MSG_CLASSIC_BTN_B: i8 = 3;
    pub const MSG_CLASSIC_BTN_PLUS: i8 = 4;
    pub const MSG_CLASSIC_BTN_MINUS: i8 = 5;
    pub const MSG_CLASSIC_BTN_LEFT: i8 = 6;
    pub const MSG_CLASSIC_BTN_RIGHT: i8 = 7;
    pub const MSG_CLASSIC_BTN_UP: i8 = 8;
    pub const MSG_CLASSIC_BTN_DOWN: i8 = 9;
    pub const MSG_CLASSIC_BTN_HOME: i8 = 10;
    pub const MSG_CLASSIC_BTN_L: i8 = 11;
    pub const MSG_CLASSIC_BTN_R: i8 = 12;
    pub const MSG_CLASSIC_BTN_ZL: i8 = 13;
    pub const MSG_CLASSIC_BTN_ZR: i8 = 14;
}

impl Default for State {
    fn default() -> Self {
        State {
            header: crate::std_msgs::msg::Header::default(),
            angular_velocity_zeroed: crate::geometry_msgs::msg::Vector3::default(),
            angular_velocity_raw: crate::geometry_msgs::msg::Vector3::default(),
            angular_velocity_covariance: [0.0; 9],
            linear_acceleration_zeroed: crate::geometry_msgs::msg::Vector3::default(),
            linear_acceleration_raw: crate::geometry_msgs::msg::Vector3::default(),
            linear_acceleration_covariance: [0.0; 9],
            nunchuk_acceleration_zeroed: crate::geometry_msgs::msg::Vector3::default(),
            nunchuk_acceleration_raw: crate::geometry_msgs::msg::Vector3::default(),
            nunchuk_joystick_zeroed: [0.0; 2],
            nunchuk_joystick_raw: [0.0; 2],
            buttons: [false; 11],
            nunchuk_buttons: [false; 2],
            leds: [false; 4],
            rumble: false,
            ir_tracking: Vec::new(),
            raw_battery: 0.0,
            percent_battery: 0.0,
            zeroing_time: crate::builtin_interfaces::msg::Time::default(),
            errors: 0,
        }
    }
}

impl ros2_client::Message for State {}
