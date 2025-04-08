use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Feedback {
    pub device_number: u8,
    pub device_name: ::std::string::String,
    pub duty_cycle: f32,
    pub current: f32,
    pub travel: f64,
    pub speed: f64,
    pub setpoint: f64,
}

impl Default for Feedback {
    fn default() -> Self {
        Feedback {
            device_number: 0,
            device_name: ::std::string::String::new(),
            duty_cycle: 0.0,
            current: 0.0,
            travel: 0.0,
            speed: 0.0,
            setpoint: 0.0,
        }
    }
}

impl ros2_client::Message for Feedback {}
