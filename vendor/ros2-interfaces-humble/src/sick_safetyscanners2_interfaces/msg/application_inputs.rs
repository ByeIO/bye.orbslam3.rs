use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApplicationInputs {
    pub unsafe_inputs_input_sources: Vec<bool>,
    pub unsafe_inputs_flags: Vec<bool>,
    pub monitoring_case_number_inputs: Vec<u16>,
    pub monitoring_case_number_inputs_flags: Vec<bool>,
    pub linear_velocity_inputs_velocity_0: i16,
    pub linear_velocity_inputs_velocity_0_valid: bool,
    pub linear_velocity_inputs_velocity_0_transmitted_safely: bool,
    pub linear_velocity_inputs_velocity_1: i16,
    pub linear_velocity_inputs_velocity_1_valid: bool,
    pub linear_velocity_inputs_velocity_1_transmitted_safely: bool,
    pub sleep_mode_input: u8,
}

impl Default for ApplicationInputs {
    fn default() -> Self {
        ApplicationInputs {
            unsafe_inputs_input_sources: Vec::new(),
            unsafe_inputs_flags: Vec::new(),
            monitoring_case_number_inputs: Vec::new(),
            monitoring_case_number_inputs_flags: Vec::new(),
            linear_velocity_inputs_velocity_0: 0,
            linear_velocity_inputs_velocity_0_valid: false,
            linear_velocity_inputs_velocity_0_transmitted_safely: false,
            linear_velocity_inputs_velocity_1: 0,
            linear_velocity_inputs_velocity_1_valid: false,
            linear_velocity_inputs_velocity_1_transmitted_safely: false,
            sleep_mode_input: 0,
        }
    }
}

impl ros2_client::Message for ApplicationInputs {}
