use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Status {
    pub device_number: u8,
    pub device_name: ::std::string::String,
    pub bus_voltage: f32,
    pub temperature: f32,
    pub output_voltage: f32,
    pub analog_input: f32,
    pub mode: u8,
    pub fault: u8,
}

impl Status {
    pub const MODE_VOLTAGE: u8 = 0;
    pub const MODE_CURRENT: u8 = 1;
    pub const MODE_SPEED: u8 = 2;
    pub const MODE_POSITION: u8 = 3;
    pub const MODE_VCOMP: u8 = 4;
    pub const FAULT_CURRENT: u8 = 1;
    pub const FAULT_TEMPERATURE: u8 = 2;
    pub const FAULT_BUS_VOLTAGE: u8 = 4;
    pub const FAULT_BRIDGE_DRIVER: u8 = 8;
}

impl Default for Status {
    fn default() -> Self {
        Status {
            device_number: 0,
            device_name: ::std::string::String::new(),
            bus_voltage: 0.0,
            temperature: 0.0,
            output_voltage: 0.0,
            analog_input: 0.0,
            mode: 0,
            fault: 0,
        }
    }
}

impl ros2_client::Message for Status {}
