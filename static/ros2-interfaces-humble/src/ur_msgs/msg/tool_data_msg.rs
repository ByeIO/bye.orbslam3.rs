use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToolDataMsg {
    pub analog_input_range2: i8,
    pub analog_input_range3: i8,
    pub analog_input2: f64,
    pub analog_input3: f64,
    pub tool_voltage_48v: f32,
    pub tool_output_voltage: u8,
    pub tool_current: f32,
    pub tool_temperature: f32,
    pub tool_mode: u8,
}

impl ToolDataMsg {
    pub const ANALOG_INPUT_RANGE_CURRENT: i8 = 0;
    pub const ANALOG_INPUT_RANGE_VOLTAGE: i8 = 1;
    pub const TOOL_BOOTLOADER_MODE: u8 = 249;
    pub const TOOL_RUNNING_MODE: u8 = 253;
    pub const TOOL_IDLE_MODE: u8 = 255;
}

impl Default for ToolDataMsg {
    fn default() -> Self {
        ToolDataMsg {
            analog_input_range2: 0,
            analog_input_range3: 0,
            analog_input2: 0.0,
            analog_input3: 0.0,
            tool_voltage_48v: 0.0,
            tool_output_voltage: 0,
            tool_current: 0.0,
            tool_temperature: 0.0,
            tool_mode: 0,
        }
    }
}

impl ros2_client::Message for ToolDataMsg {}
