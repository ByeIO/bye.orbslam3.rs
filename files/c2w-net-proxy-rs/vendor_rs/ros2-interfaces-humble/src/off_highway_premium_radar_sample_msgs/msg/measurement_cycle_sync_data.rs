use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MeasurementCycleSyncData {
    pub sync: bool,
    pub sensor_time_offset: crate::builtin_interfaces::msg::Time,
}

impl Default for MeasurementCycleSyncData {
    fn default() -> Self {
        MeasurementCycleSyncData {
            sync: false,
            sensor_time_offset: crate::builtin_interfaces::msg::Time::default(),
        }
    }
}

impl ros2_client::Message for MeasurementCycleSyncData {}
