use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeviceStatus {
    pub header: crate::std_msgs::msg::Header,
    pub status: u8,
    pub general_status: crate::sick_safevisionary_interfaces::msg::GeneralStatus,
    pub cop_safety_related: u32,
    pub cop_non_safety_related: u32,
    pub cop_reset_required: u32,
    pub active_monitoring_case: crate::sick_safevisionary_interfaces::msg::ActiveMonitoringCase,
    pub contamination_level: u8,
}

impl Default for DeviceStatus {
    fn default() -> Self {
        DeviceStatus {
            header: crate::std_msgs::msg::Header::default(),
            status: 0,
            general_status: crate::sick_safevisionary_interfaces::msg::GeneralStatus::default(),
            cop_safety_related: 0,
            cop_non_safety_related: 0,
            cop_reset_required: 0,
            active_monitoring_case: crate::sick_safevisionary_interfaces::msg::ActiveMonitoringCase::default(),
            contamination_level: 0,
        }
    }
}

impl ros2_client::Message for DeviceStatus {}
