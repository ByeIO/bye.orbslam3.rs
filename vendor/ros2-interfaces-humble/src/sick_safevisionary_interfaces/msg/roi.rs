use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ROI {
    pub id: u8,
    pub result_data: crate::sick_safevisionary_interfaces::msg::ROIObservationResultData,
    pub safety_data: crate::sick_safevisionary_interfaces::msg::ROIObservationSafetyData,
    pub distance_value: u16,
}

impl Default for ROI {
    fn default() -> Self {
        ROI {
            id: 0,
            result_data: crate::sick_safevisionary_interfaces::msg::ROIObservationResultData::default(),
            safety_data: crate::sick_safevisionary_interfaces::msg::ROIObservationSafetyData::default(),
            distance_value: 0,
        }
    }
}

impl ros2_client::Message for ROI {}
