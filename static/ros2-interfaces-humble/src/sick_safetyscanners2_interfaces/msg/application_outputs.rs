use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApplicationOutputs {
    pub evaluation_path_outputs_eval_out: Vec<bool>,
    pub evaluation_path_outputs_is_safe: Vec<bool>,
    pub evaluation_path_outputs_is_valid: Vec<bool>,
    pub monitoring_case_number_outputs: Vec<u16>,
    pub monitoring_case_number_outputs_flags: Vec<bool>,
    pub sleep_mode_output: u8,
    pub sleep_mode_output_valid: bool,
    pub error_flag_contamination_warning: bool,
    pub error_flag_contamination_error: bool,
    pub error_flag_manipulation_error: bool,
    pub error_flag_glare: bool,
    pub error_flag_reference_contour_intruded: bool,
    pub error_flag_critical_error: bool,
    pub error_flags_are_valid: bool,
    pub linear_velocity_outputs_velocity_0: i16,
    pub linear_velocity_outputs_velocity_0_valid: bool,
    pub linear_velocity_outputs_velocity_0_transmitted_safely: bool,
    pub linear_velocity_outputs_velocity_1: i16,
    pub linear_velocity_outputs_velocity_1_valid: bool,
    pub linear_velocity_outputs_velocity_1_transmitted_safely: bool,
    pub resulting_velocity: Vec<i16>,
    pub resulting_velocity_flags: Vec<bool>,
}

impl Default for ApplicationOutputs {
    fn default() -> Self {
        ApplicationOutputs {
            evaluation_path_outputs_eval_out: Vec::new(),
            evaluation_path_outputs_is_safe: Vec::new(),
            evaluation_path_outputs_is_valid: Vec::new(),
            monitoring_case_number_outputs: Vec::new(),
            monitoring_case_number_outputs_flags: Vec::new(),
            sleep_mode_output: 0,
            sleep_mode_output_valid: false,
            error_flag_contamination_warning: false,
            error_flag_contamination_error: false,
            error_flag_manipulation_error: false,
            error_flag_glare: false,
            error_flag_reference_contour_intruded: false,
            error_flag_critical_error: false,
            error_flags_are_valid: false,
            linear_velocity_outputs_velocity_0: 0,
            linear_velocity_outputs_velocity_0_valid: false,
            linear_velocity_outputs_velocity_0_transmitted_safely: false,
            linear_velocity_outputs_velocity_1: 0,
            linear_velocity_outputs_velocity_1_valid: false,
            linear_velocity_outputs_velocity_1_transmitted_safely: false,
            resulting_velocity: Vec::new(),
            resulting_velocity_flags: Vec::new(),
        }
    }
}

impl ros2_client::Message for ApplicationOutputs {}
