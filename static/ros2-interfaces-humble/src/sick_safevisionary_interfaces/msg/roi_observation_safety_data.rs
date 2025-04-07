use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ROIObservationSafetyData {
    pub invalid_due_to_invalid_pixels: u8,
    pub invalid_due_to_variance: u8,
    pub invalid_due_to_overexposure: u8,
    pub invalid_due_to_underexposure: u8,
    pub invalid_due_to_temporal_variance: u8,
    pub invalid_due_to_outside_of_measurement_range: u8,
    pub invalid_due_to_retro_reflector_interference: u8,
    pub contamination_error: u8,
    pub quality_class: u8,
    pub slot_active: u8,
}

impl Default for ROIObservationSafetyData {
    fn default() -> Self {
        ROIObservationSafetyData {
            invalid_due_to_invalid_pixels: 0,
            invalid_due_to_variance: 0,
            invalid_due_to_overexposure: 0,
            invalid_due_to_underexposure: 0,
            invalid_due_to_temporal_variance: 0,
            invalid_due_to_outside_of_measurement_range: 0,
            invalid_due_to_retro_reflector_interference: 0,
            contamination_error: 0,
            quality_class: 0,
            slot_active: 0,
        }
    }
}

impl ros2_client::Message for ROIObservationSafetyData {}
