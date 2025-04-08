use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MeasurementData {
    pub number_of_beams: u32,
    pub scan_points: Vec<crate::sick_safetyscanners2_interfaces::msg::ScanPoint>,
}

impl Default for MeasurementData {
    fn default() -> Self {
        MeasurementData {
            number_of_beams: 0,
            scan_points: Vec::new(),
        }
    }
}

impl ros2_client::Message for MeasurementData {}
