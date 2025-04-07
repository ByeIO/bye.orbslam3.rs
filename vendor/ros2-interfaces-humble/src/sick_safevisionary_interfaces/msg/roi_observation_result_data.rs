use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ROIObservationResultData {
    pub task_result: u8,
    pub result_safe: u8,
    pub result_valid: u8,
    pub distance_valid: u8,
    pub distance_safe: u8,
}

impl Default for ROIObservationResultData {
    fn default() -> Self {
        ROIObservationResultData {
            task_result: 0,
            result_safe: 0,
            result_valid: 0,
            distance_valid: 0,
            distance_safe: 0,
        }
    }
}

impl ros2_client::Message for ROIObservationResultData {}
