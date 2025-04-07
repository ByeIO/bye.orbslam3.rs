use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MasterboardDataMsg {
    pub digital_input_bits: u32,
    pub digital_output_bits: u32,
    pub analog_input_range0: i8,
    pub analog_input_range1: i8,
    pub analog_input0: f64,
    pub analog_input1: f64,
    pub analog_output_domain0: i8,
    pub analog_output_domain1: i8,
    pub analog_output0: f64,
    pub analog_output1: f64,
    pub masterboard_temperature: f32,
    pub robot_voltage_48v: f32,
    pub robot_current: f32,
    pub master_io_current: f32,
    pub master_safety_state: u8,
    pub master_onoff_state: u8,
}

impl Default for MasterboardDataMsg {
    fn default() -> Self {
        MasterboardDataMsg {
            digital_input_bits: 0,
            digital_output_bits: 0,
            analog_input_range0: 0,
            analog_input_range1: 0,
            analog_input0: 0.0,
            analog_input1: 0.0,
            analog_output_domain0: 0,
            analog_output_domain1: 0,
            analog_output0: 0.0,
            analog_output1: 0.0,
            masterboard_temperature: 0.0,
            robot_voltage_48v: 0.0,
            robot_current: 0.0,
            master_io_current: 0.0,
            master_safety_state: 0,
            master_onoff_state: 0,
        }
    }
}

impl ros2_client::Message for MasterboardDataMsg {}
