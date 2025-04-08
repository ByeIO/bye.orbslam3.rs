use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClockSteering {
    pub source: ::std::string::String,
    pub steering_state: ::std::string::String,
    pub period: u32,
    pub pulse_width: f64,
    pub bandwidth: f64,
    pub slope: f32,
    pub offset: f64,
    pub drift_rate: f64,
}

impl ClockSteering {
    pub const INTERNAL_SOURCE: i8 = 0;
    pub const EXTERNAL_SOURCE: i8 = 1;
    pub const FIRST_ORDER_STEERING_STATE: i8 = 0;
    pub const SECOND_ORDER_STEERING_STATE: i8 = 1;
    pub const CALIBRATE_HIGH_STEERING_STATE: i8 = 2;
    pub const CALIBRATE_LOW_STEERING_STATE: i8 = 3;
    pub const CALIBRATE_CENTER_STEERING_STATE: i8 = 4;
}

impl Default for ClockSteering {
    fn default() -> Self {
        ClockSteering {
            source: ::std::string::String::new(),
            steering_state: ::std::string::String::new(),
            period: 0,
            pulse_width: 0.0,
            bandwidth: 0.0,
            slope: 0.0,
            offset: 0.0,
            drift_rate: 0.0,
        }
    }
}

impl ros2_client::Message for ClockSteering {}
