use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NovatelReceiverStatus {
    pub original_status_code: u32,
    pub error_flag: bool,
    pub temperature_flag: bool,
    pub voltage_supply_flag: bool,
    pub antenna_powered: bool,
    pub antenna_is_open: bool,
    pub antenna_is_shorted: bool,
    pub cpu_overload_flag: bool,
    pub com1_buffer_overrun: bool,
    pub com2_buffer_overrun: bool,
    pub com3_buffer_overrun: bool,
    pub usb_buffer_overrun: bool,
    pub rf1_agc_flag: bool,
    pub rf2_agc_flag: bool,
    pub almanac_flag: bool,
    pub position_solution_flag: bool,
    pub position_fixed_flag: bool,
    pub clock_steering_status_enabled: bool,
    pub clock_model_flag: bool,
    pub oemv_external_oscillator_flag: bool,
    pub software_resource_flag: bool,
    pub aux1_status_event_flag: bool,
    pub aux2_status_event_flag: bool,
    pub aux3_status_event_flag: bool,
}

impl Default for NovatelReceiverStatus {
    fn default() -> Self {
        NovatelReceiverStatus {
            original_status_code: 0,
            error_flag: false,
            temperature_flag: false,
            voltage_supply_flag: false,
            antenna_powered: false,
            antenna_is_open: false,
            antenna_is_shorted: false,
            cpu_overload_flag: false,
            com1_buffer_overrun: false,
            com2_buffer_overrun: false,
            com3_buffer_overrun: false,
            usb_buffer_overrun: false,
            rf1_agc_flag: false,
            rf2_agc_flag: false,
            almanac_flag: false,
            position_solution_flag: false,
            position_fixed_flag: false,
            clock_steering_status_enabled: false,
            clock_model_flag: false,
            oemv_external_oscillator_flag: false,
            software_resource_flag: false,
            aux1_status_event_flag: false,
            aux2_status_event_flag: false,
            aux3_status_event_flag: false,
        }
    }
}

impl ros2_client::Message for NovatelReceiverStatus {}
