use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeneralSystemState {
    pub run_mode_active: bool,
    pub standby_mode_active: bool,
    pub contamination_warning: bool,
    pub contamination_error: bool,
    pub reference_contour_status: bool,
    pub manipulation_status: bool,
    pub safe_cut_off_path: Vec<bool>,
    pub non_safe_cut_off_path: Vec<bool>,
    pub reset_required_cut_off_path: Vec<bool>,
    pub current_monitoring_case_no_table_1: u8,
    pub current_monitoring_case_no_table_2: u8,
    pub current_monitoring_case_no_table_3: u8,
    pub current_monitoring_case_no_table_4: u8,
    pub application_error: bool,
    pub device_error: bool,
}

impl Default for GeneralSystemState {
    fn default() -> Self {
        GeneralSystemState {
            run_mode_active: false,
            standby_mode_active: false,
            contamination_warning: false,
            contamination_error: false,
            reference_contour_status: false,
            manipulation_status: false,
            safe_cut_off_path: Vec::new(),
            non_safe_cut_off_path: Vec::new(),
            reset_required_cut_off_path: Vec::new(),
            current_monitoring_case_no_table_1: 0,
            current_monitoring_case_no_table_2: 0,
            current_monitoring_case_no_table_3: 0,
            current_monitoring_case_no_table_4: 0,
            application_error: false,
            device_error: false,
        }
    }
}

impl ros2_client::Message for GeneralSystemState {}
