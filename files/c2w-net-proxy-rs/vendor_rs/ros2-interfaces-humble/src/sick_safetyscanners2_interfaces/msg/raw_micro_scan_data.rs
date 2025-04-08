use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RawMicroScanData {
    pub header: crate::sick_safetyscanners2_interfaces::msg::DataHeader,
    pub derived_values: crate::sick_safetyscanners2_interfaces::msg::DerivedValues,
    pub general_system_state: crate::sick_safetyscanners2_interfaces::msg::GeneralSystemState,
    pub measurement_data: crate::sick_safetyscanners2_interfaces::msg::MeasurementData,
    pub intrusion_data: crate::sick_safetyscanners2_interfaces::msg::IntrusionData,
    pub application_data: crate::sick_safetyscanners2_interfaces::msg::ApplicationData,
}

impl Default for RawMicroScanData {
    fn default() -> Self {
        RawMicroScanData {
            header: crate::sick_safetyscanners2_interfaces::msg::DataHeader::default(),
            derived_values: crate::sick_safetyscanners2_interfaces::msg::DerivedValues::default(),
            general_system_state: crate::sick_safetyscanners2_interfaces::msg::GeneralSystemState::default(),
            measurement_data: crate::sick_safetyscanners2_interfaces::msg::MeasurementData::default(),
            intrusion_data: crate::sick_safetyscanners2_interfaces::msg::IntrusionData::default(),
            application_data: crate::sick_safetyscanners2_interfaces::msg::ApplicationData::default(),
        }
    }
}

impl ros2_client::Message for RawMicroScanData {}
